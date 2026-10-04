import json
import sys
import time
import subprocess

mode = sys.argv[1] if len(sys.argv) > 1 else "symbols"
counter = sys.argv[2] if len(sys.argv) > 2 else None
if counter and mode == "symbols":
    try:
        starts = int(open(counter, encoding="utf-8").read())
    except (FileNotFoundError, ValueError):
        starts = 0
    with open(counter, "w", encoding="utf-8") as output:
        output.write(str(starts + 1))


def read_message():
    headers = {}
    while True:
        line = sys.stdin.buffer.readline()
        if not line:
            return None
        if line in (b"\r\n", b"\n"):
            break
        name, value = line.decode("ascii").split(":", 1)
        headers[name.lower()] = value.strip()
    return json.loads(sys.stdin.buffer.read(int(headers["content-length"])))


def send(value):
    body = json.dumps(value, separators=(",", ":")).encode()
    sys.stdout.buffer.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
    sys.stdout.buffer.flush()


while True:
    message = read_message()
    if message is None or message.get("method") == "exit":
        break
    method = message.get("method")
    if "id" not in message:
        if mode == "diagnostics" and method == "textDocument/didOpen":
            uri = message["params"]["textDocument"]["uri"]
            send(
                {
                    "jsonrpc": "2.0",
                    "method": "textDocument/publishDiagnostics",
                    "params": {
                        "uri": uri,
                        "diagnostics": [
                            {
                                "range": {
                                    "start": {"line": 0, "character": 4},
                                    "end": {"line": 0, "character": 6},
                                },
                                "severity": 1,
                                "message": "fixture diagnostic",
                            }
                        ],
                    },
                }
            )
        continue
    if method == "initialize" and mode == "initialize-crash":
        sys.exit(4)
    elif method == "initialize":
        result = {
            "capabilities": {"documentSymbolProvider": True},
            "serverInfo": {"name": "fake-lsp", "version": "fixture-1"},
        }
    elif method == "textDocument/documentSymbol" and mode == "crash":
        sys.exit(3)
    elif method == "textDocument/documentSymbol" and mode == "timeout":
        time.sleep(5)
        continue
    elif method == "textDocument/documentSymbol" and mode == "descendant-timeout":
        subprocess.Popen([
            sys.executable,
            "-c",
            "import sys,time; time.sleep(0.5); open(sys.argv[1], 'w').write('alive')",
            counter,
        ])
        time.sleep(5)
        continue
    elif method == "textDocument/documentSymbol":
        result = [
            {
                "name": "answer",
                "kind": 12,
                "detail": "pub fn answer() -> i32",
                "range": {
                    "start": {"line": 0, "character": 0},
                    "end": {"line": 0, "character": 0},
                },
            }
        ]
    else:
        result = None
    send({"jsonrpc": "2.0", "id": message["id"], "result": result})
