"use strict";
(function () {
  if (!document.modelContext || typeof document.modelContext.registerTool !== "function") return;
  function load() {
    return fetch(new URL("webmcp.json", document.baseURI), {credentials: "same-origin"})
      .then(function (response) { if (!response.ok) throw new Error("WebMCP manifest returned " + response.status); return response.json(); });
  }
  function page(manifest, path) {
    var current = path || window.location.pathname + window.location.hash;
    current = current.replace(/^\//, "");
    return (manifest.pages || []).find(function (item) {
      return item.url === current || item.url === window.location.pathname.replace(/^\//, "");
    }) || null;
  }
  function localSearch(index, query) {
    var terms = String(query || "").toLowerCase().split(/\s+/).filter(Boolean);
    return Object.keys(index.documents || {}).map(function (id) { return index.documents[id]; })
      .map(function (document) {
        var title = String(document.title || "").toLowerCase();
        var content = String(document.content || "").toLowerCase();
        var score = terms.reduce(function (total, term) {
          return total + (title.indexOf(term) >= 0 ? 3 : 0) + (content.indexOf(term) >= 0 ? 1 : 0);
        }, 0);
        return {document: document, score: score};
      })
      .filter(function (item) { return terms.length === 0 || item.score > 0; })
      .sort(function (left, right) { return right.score - left.score || left.document.id.localeCompare(right.document.id); })
      .map(function (item) {
        return {id: item.document.id, title: item.document.title, url: item.document.url || null, content: item.document.content, relevance_score: item.score};
      });
  }
  function loadLocalIndex(manifest) {
    if (!manifest.search || !manifest.search.docindex || !manifest.search.docindex.enabled) return Promise.resolve(null);
    return fetch(new URL(manifest.search.docindex.artifact, document.baseURI), {credentials: "same-origin"})
      .then(function (response) { if (!response.ok) throw new Error("DocIndex artifact returned " + response.status); return response.json(); });
  }
  load().then(function (manifest) {
    var tools = [
      {name: "sphinx.get_page_context", description: "Return documentation page context.", inputSchema: {type: "object", properties: {path: {type: "string"}}}, execute: function (input) { return Promise.resolve(page(manifest, input && input.path)); }},
      {name: "sphinx.list_navigation", description: "Return documentation navigation.", inputSchema: {type: "object", properties: {}}, execute: function () { return Promise.resolve(manifest.navigation); }},
      {name: "sphinx.get_documentation_metadata", description: "Return documentation metadata and artifacts.", inputSchema: {type: "object", properties: {}}, execute: function () { return Promise.resolve(manifest); }},
      {name: "sphinx.search", description: "Search the documentation.", inputSchema: {type: "object", properties: {query: {type: "string"}}, required: ["query"]}, execute: function (input) { return loadLocalIndex(manifest).then(function (index) { if (index) return {backend: "docindex", results: localSearch(index, input && input.query)}; var url = new URL(manifest.search.native.page, document.baseURI); url.searchParams.set("q", input && input.query || ""); window.location.assign(url.href); return {backend: "native", url: url.href}; }).catch(function () { var url = new URL(manifest.search.native.page, document.baseURI); url.searchParams.set("q", input && input.query || ""); window.location.assign(url.href); return {backend: "native", url: url.href}; }); }},
      {name: "sphinx.navigate", description: "Navigate to a same-origin documentation page.", inputSchema: {type: "object", properties: {path: {type: "string"}}, required: ["path"]}, execute: function (input) { var url = new URL(input.path, document.baseURI); if (url.origin !== window.location.origin) throw new Error("navigation must remain same-origin"); window.location.assign(url.href); return Promise.resolve({url: url.href}); }}
    ];
    return Promise.all(tools.map(function (tool) { return document.modelContext.registerTool(tool, {exposedTo: manifest.webmcp.exposed_to || []}); }));
  }).catch(function () {});
}());
