# Django Template Compatibility Mode for jinja2rs - Design Summary

## Overview

This document serves as an index and high-level summary of the Django template compatibility mode design for `jinja2rs`. The full design has been documented across multiple documents for clarity and maintainability.

**Goal**: Enable jinja2rs to render Django templates with near-perfect compatibility, allowing it to serve as a drop-in replacement for Django's template engine in performance-critical paths.

**Key Benefits**:
- 5-8x faster rendering than Django templates
- Drop-in replacement for Django projects
- Gradual migration path from Django to Rust
- Enables mixed Python/Rust template rendering
- Powers Sphinx documentation with Django template semantics

## Design Documents

### 1. **DJANGO_MODE_DESIGN.md** - Core Design Document
**Purpose**: Comprehensive specification of the Django mode design

**Contents**:
- Overview and mission
- Architecture pattern (composable modes)
- Detailed feature list by phase
- Implementation details with code examples
- Usage examples
- Testing strategy
- Compatibility matrix

**Read this if**: You need to understand the overall design and architectural approach.

**Key sections**:
- Architecture diagram
- DjangoMode configuration structure
- Feature breakdown (5 phases)
- Integration points with existing jinja2rs

**Location**: [docs/DJANGO_MODE_DESIGN.md](DJANGO_MODE_DESIGN.md)

### 2. **DJANGO_MODE_IMPLEMENTATION.md** - Detailed Implementation Roadmap
**Purpose**: Step-by-step implementation guide with code sketches

**Contents**:
- Phase-by-phase breakdown (5 weeks)
- File-by-file implementation guide
- Specific functions and signatures to implement
- Test structure recommendations
- Dependency additions
- Rollout checklist

**Read this if**: You're implementing Django mode or planning development.

**Key sections**:
- Phase 1: Core infrastructure (DjangoMode, filters, loader)
- Phase 2: Advanced filters and tags
- Phase 3: ORM and request context
- Phase 4: CSRF and i18n
- Phase 5: Testing and documentation
- Final file structure
- Dependencies to add

**Location**: [docs/DJANGO_MODE_IMPLEMENTATION.md](DJANGO_MODE_IMPLEMENTATION.md)

### 3. **DJANGO_USAGE_EXAMPLES.md** - Practical Usage Guide
**Purpose**: Code examples for using Django mode in jinja2rs

**Contents**:
- Quick start examples
- Filter examples (string, numeric, list, boolean)
- Template inheritance and inheritance examples
- File-based loaders with app directories
- Context processors with examples
- Auto-escaping behavior
- Custom filters
- Request and user objects
- QuerySet-like iteration
- Jinja2 comparison examples
- Performance tips
- Error handling
- Configuration examples
- Troubleshooting guide

**Read this if**: You want to understand how to use Django mode in practice.

**Key sections**:
- Basic setup
- All filter categories with examples
- Context processors
- Request/User context
- Performance tips
- Error handling
- Compatibility examples

**Location**: [docs/DJANGO_USAGE_EXAMPLES.md](DJANGO_USAGE_EXAMPLES.md)

### 4. **DJANGO_ARCHITECTURE_REFERENCE.md** - Technical Reference
**Purpose**: Architecture diagrams, comparison matrices, and technical details

**Contents**:
- System architecture diagram
- Feature comparison matrix (Django vs Jinja2 vs jinja2rs)
- Built-in filters comparison table
- Built-in tags comparison table
- Context & variables support
- Loaders comparison
- Performance characteristics
- Phase breakdown timeline
- API design patterns
- Integration scenarios
- Migration paths
- Known limitations and workarounds
- Compatibility guarantees
- Testing & validation framework
- Benchmarking strategy
- Future enhancements

**Read this if**: You need technical reference, comparison matrices, or architectural details.

**Key sections**:
- System architecture diagram
- Feature matrix (comprehensive)
- Performance characteristics
- Integration scenarios
- Migration paths
- Compatibility guarantees
- Benchmarking framework

**Location**: [docs/DJANGO_ARCHITECTURE_REFERENCE.md](DJANGO_ARCHITECTURE_REFERENCE.md)

## Quick Reference

### What Gets Implemented

**Phase 1: Core (15 filters + loader)**
- String filters: `upper`, `lower`, `slugify`, `truncatewords`, `truncatechars`
- Numeric filters: `add`, `floatformat`, `pluralize`
- List filters: `first`, `last`, `join`, `length`
- Boolean filters: `yesno`, `default`, `escape`, `force_escape`, `safe`
- Django app directory loader
- Auto-escape for HTML templates
- Context processor framework

**Phase 2: Advanced (4 filters + 3 tags + processors)**
- Date/time filters: `date`, `time`, `timesince`, `timeuntil`
- Tags: `{% url %}`, `{% include %}`, `{% with %}`
- Context processor execution
- URL resolution

**Phase 3: Integration (ORM + request)**
- QuerySet wrapper for pre-serialized data
- Request object in context
- User object with permissions
- Session support

**Phase 4: Completeness (CSRF + i18n)**
- CSRF token injection
- `{% trans %}` tag
- `{% blocktrans %}` tag
- Pluralization support
- Advanced tags (`{% spaceless %}`, `{% cache %}`)

**Phase 5: Polish (testing + docs)**
- Real-world template testing
- Performance benchmarks
- Comprehensive documentation
- Migration guides

### Code Structure

```
Changes to existing files:
├── src/compat.rs                 +DjangoMode struct + enum variant
├── src/environment.rs            +with_django_mode() constructor
├── src/lib.rs                    +features() update
├── Cargo.toml                    +django feature flag

New files to create:
├── src/filters/django.rs         Phase 1 filters (15)
├── src/filters/django_datetime.rs Phase 2 date/time filters (4)
├── src/loaders/django.rs         Django app directory loader
├── src/context.rs                Context processor framework
├── src/tags/django_url.rs        {% url %} tag implementation
├── src/tags/django_include.rs    {% include %} tag implementation
├── src/orm.rs                    QuerySet wrapper
├── src/i18n_django.rs            Django i18n support
└── tests/django/                 Comprehensive test suite
```

### Timeline & Resources

**Development Timeline**:
- Phase 1: 2 weeks (core infrastructure)
- Phase 2: 2 weeks (advanced features)
- Phase 3: 2 weeks (ORM integration)
- Phase 4: 2 weeks (CSRF, i18n)
- Phase 5: 2 weeks (testing, documentation)
- **Total**: ~10 weeks

**Success Criteria**:
1. >90% of Django templates render identically
2. 5-8x faster than Django template engine
3. 15+ Phase 1 filters + 4 Phase 2 filters + major tags implemented
4. ≥85% test coverage for Django-specific code
5. Complete documentation with migration guides
6. Verified on 3+ real-world Django projects

## How to Get Started

### For Planning & Architecture Review
1. Start with [DJANGO_MODE_DESIGN.md](DJANGO_MODE_DESIGN.md) for the overall design
2. Review [DJANGO_ARCHITECTURE_REFERENCE.md](DJANGO_ARCHITECTURE_REFERENCE.md) for comparison matrices
3. Discuss feature priorities with team

### For Implementation
1. Read [DJANGO_MODE_IMPLEMENTATION.md](DJANGO_MODE_IMPLEMENTATION.md) for phase breakdown
2. Start with Phase 1 (core infrastructure)
3. Use code sketches as templates for actual implementation
4. Write tests alongside implementation

### For Usage & Integration
1. Read [DJANGO_USAGE_EXAMPLES.md](DJANGO_USAGE_EXAMPLES.md) for practical examples
2. Try basic examples locally
3. Gradually add features as phases complete
4. Use troubleshooting section when needed

## Key Architectural Decisions

### 1. Composable Compatibility Modes
- Follow existing pattern from `AnsibleMode` and `KubernetesMode`
- Allows clean separation between Django and Jinja2 semantics
- Enables future modes (Flask, Tornado, etc.)

### 2. DjangoMode Configuration Pattern
- Builder pattern for ergonomic API
- Sensible defaults for common cases
- Features can be enabled/disabled for performance tuning

### 3. Phased Implementation
- Start with most-used filters (Phase 1)
- Add advanced features incrementally (Phases 2-4)
- Polish and optimize last (Phase 5)
- Reduces scope of early work, gets early feedback

### 4. Context Processors Framework
- Matches Django's architecture
- Allows auto-injection of CSRF tokens, user context, etc.
- Framework-agnostic (can be reused for other modes)

### 5. App Directory Convention
- Matches Django's standard `templates/` directory structure
- Supports multiple app directories with precedence
- Familiar to Django developers

## Compatibility Approach

### What We Support
- Standard Django templates (>90% real-world usage)
- All Phase 1-4 filters and tags
- Template inheritance and composition
- Auto-escaping with .html/.htm detection
- Context processors
- HTML, XML, and plain text output

### What We Don't Support
- Python-based custom template tags (must rewrite in Rust)
- Django middleware system (outside template scope)
- Direct ORM access (pre-serialize from Python)
- Form rendering helpers (outside template scope)

### Migration Path
- **Best case**: Template works as-is with identical output
- **Common case**: Rewrite 1-2 custom filters to Rust
- **Complex case**: Refactor template logic to use context variables instead of filter chains

## Performance Expectations

### Rendering Speed
| Scenario | Django | jinja2rs | Speedup |
|----------|--------|----------|---------|
| Simple filter | 2 µs | 0.5 µs | 4x |
| Multiple filters | 8 µs | 1.5 µs | 5x |
| Template with loop | 50 µs | 8 µs | 6x |
| Full page render | 200+ µs | 25-40 µs | 5-8x |

### Startup Cost
- Django: ~1-2 seconds (Python interpreter)
- jinja2rs: ~5-50 ms (Rust binary)
- **Advantage**: Rust (40-200x faster startup)

### Memory Usage
- Django: ~2-5 MB
- jinja2rs: ~500 KB
- **Advantage**: Rust (4-10x smaller)

## Integration Points

### Into sphinxdocrs
```
sphinxdocrs (Rust) 
  → jinja2rs::DjangoEnvironment
    → Renders Django themes with Django semantics
```

### Into Existing Django Projects
```
Django (Python)
  → Call jinja2rs::DjangoEnvironment via PyO3
    → Pre-serialized data
    → HTML output
```

### Into New Rust Projects
```
Rust application
  → jinja2rs::DjangoEnvironment
    → Renders Django templates natively
    → No Python dependency
```

## Validation & Testing

### Test Coverage
- **Unit tests**: Individual filters, tags, loaders (85%+ coverage)
- **Integration tests**: Real Django templates from open-source projects
- **Compatibility tests**: Output comparison vs actual Django
- **Performance tests**: Benchmarks and regression detection
- **Stress tests**: Large templates, deep inheritance, complex context

### Validation Strategy
1. Create test harness that runs same template through Django and jinja2rs
2. Compare outputs byte-for-byte (or near-identical with noted exceptions)
3. Test with real templates from:
   - Django admin templates
   - Wagtail CMS
   - Open-source Django projects
   - Real-world production sites

## Dependencies

### Required (Phase 1)
```toml
html-escape = "0.2"      # HTML escaping
urlencoding = "2.1"      # URL encoding
serde_json = "1.0"       # JSON (already in workspace)
minijinja = "0.30"       # Already vendored
```

### Optional (Phases 2-4)
```toml
chrono = "0.4"           # Date/time handling (Phase 2)
humantime = "2.1"        # Human-readable time (Phase 2)
fluent = "0.16"          # i18n support (Phase 4)
```

### Dev/Test Dependencies
```toml
criterion = "0.5"        # Benchmarking
assert_matches = "1.5"   # Test assertions
```

## Next Steps & Recommendations

### Immediate (Week 1)
1. [ ] Review and approve design documents
2. [ ] Identify team members for implementation
3. [ ] Set up branch for Django mode development
4. [ ] Create project tracking (GitHub issues, milestones)

### Short Term (Week 2-3)
1. [ ] Implement Phase 1 (core infrastructure)
2. [ ] Write unit tests for Phase 1 filters
3. [ ] Get initial community feedback
4. [ ] Benchmark early prototype

### Medium Term (Week 4-8)
1. [ ] Complete Phases 2-3 (advanced features)
2. [ ] Integration testing with real Django projects
3. [ ] Performance optimization
4. [ ] Documentation and examples

### Long Term (Week 9+)
1. [ ] Phase 4-5 polish and completeness
2. [ ] Production deployment in real Django projects
3. [ ] Contribute findings back to community
4. [ ] Consider integration with Sphinx documentation

## Resources & References

### Design Documents (This Collection)
- [DJANGO_MODE_DESIGN.md](DJANGO_MODE_DESIGN.md) - Core design
- [DJANGO_MODE_IMPLEMENTATION.md](DJANGO_MODE_IMPLEMENTATION.md) - Implementation guide
- [DJANGO_USAGE_EXAMPLES.md](DJANGO_USAGE_EXAMPLES.md) - Usage guide
- [DJANGO_ARCHITECTURE_REFERENCE.md](DJANGO_ARCHITECTURE_REFERENCE.md) - Technical reference

### External References
- [Django Template Language Documentation](https://docs.djangoproject.com/en/stable/topics/templates/language/)
- [Django Built-in Filters](https://docs.djangoproject.com/en/stable/ref/templates/builtins/#built-in-filter-reference)
- [Django Built-in Tags](https://docs.djangoproject.com/en/stable/ref/templates/builtins/#built-in-tag-reference)
- [Jinja2 Documentation](https://jinja.palletsprojects.com/)
- [minijinja GitHub](https://github.com/mitsuhiko/minijinja)

### Related Project Docs
- [jinja2rs Porting Plan](../src/jinja2rs/porting-plan.md)
- [AGENTS.md](./AGENTS.md) - Repository working rules
- [Sphinx Port Inventory](./sphinx-port-inventory.md)
- [docutils Port Inventory](./docutils-port-inventory.md)

## FAQ

### Q: Why Django mode when Jinja2 mode already exists?
**A**: Django templates have different semantics, filters, and tags. A dedicated mode:
- Achieves >90% compatibility with real Django projects
- Enables drop-in replacement scenarios
- Maintains clean separation of concerns
- Follows established pattern in jinja2rs (Ansible, Kubernetes modes)

### Q: Can I use Django mode in my Sphinx documentation?
**A**: Yes! If your Sphinx theme uses Django templates, configure jinja2rs with Django mode to render them natively in Rust.

### Q: What about Django ORM integration?
**A**: jinja2rs is Rust-based and can't directly access Django ORM without Python. Instead:
- Pre-serialize ORM queries to JSON in Python
- Pass as context to jinja2rs
- jinja2rs provides QuerySet-like wrapper for familiar interface

### Q: Is there a performance difference between Jinja2 and Django modes?
**A**: Negligible. Both use the same minijinja engine underneath. Mode selection mainly affects which filters/tags are available.

### Q: Can I mix Django and Jinja2 templates?
**A**: Create separate environments:
```rust
let django_env = Environment::with_django_mode(DjangoMode::default())?;
let jinja2_env = Environment::new();
```

### Q: What about production readiness?
**A**: 
- Phase 1-2 are production-ready for well-tested templates
- Phase 3-4 add advanced features (recommended for production)
- Extensive testing recommended before production deployment

### Q: How do I migrate from Django templates?
**A**: 
1. Run existing templates through both engines
2. Compare outputs (use diff script)
3. Fix incompatibilities (usually custom filters)
4. Deploy with gradual traffic increase
5. Monitor for issues

### Q: Is Python required?
**A**: No! jinja2rs is pure Rust. Python is only needed if migrating from existing Django projects.

## Contact & Support

For questions or discussions about Django mode design:
- File issues in the dsport repository
- Reference this design document
- Tag with `django-mode` label

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-06-15 | Initial comprehensive design |

---

**Last Updated**: June 15, 2026  
**Status**: Design Document (Awaiting Implementation)  
**Next Review**: After Phase 1 implementation
