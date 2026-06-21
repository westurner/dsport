use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use jinja2rs::environment::Environment;
use std::hint::black_box;

/// Compile a simple template
fn simple_template() -> &'static str {
    "Hello {{ name }}!"
}

/// Compile a medium-complexity template
fn medium_template() -> &'static str {
    r#"
{% for item in items %}
  {% if item.active %}
    <li>{{ item.name }}</li>
  {% endif %}
{% endfor %}
"#
}

/// Compile a template with multiple filters
fn filters_template() -> &'static str {
    "{{ text | upper | wordwrap(40) | indent(2) }}"
}

/// Compile a template with includes/inheritance (single template)
fn with_macros_template() -> &'static str {
    r#"
{% macro render_item(item) %}
  <div class="item">
    <h3>{{ item.title }}</h3>
    <p>{{ item.description }}</p>
  </div>
{% endmacro %}

{% for item in items %}
  {{ render_item(item) }}
{% endfor %}
"#
}

fn compile_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile");

    // Compile simple template
    group.bench_function("simple", |b| {
        let env = Environment::new();
        b.iter(|| {
            black_box(env.render_str(
                black_box(simple_template()),
                black_box(&serde_json::json!({})),
            ))
        });
    });

    // Compile medium template
    group.bench_function("medium", |b| {
        let env = Environment::new();
        b.iter(|| {
            black_box(env.render_str(
                black_box(medium_template()),
                black_box(&serde_json::json!({"items": []})),
            ))
        });
    });

    // Compile template with filters
    group.bench_function("with_filters", |b| {
        let env = Environment::new();
        b.iter(|| {
            black_box(env.render_str(
                black_box(filters_template()),
                black_box(&serde_json::json!({"text": "hello"})),
            ))
        });
    });

    // Compile template with macros
    group.bench_function("with_macros", |b| {
        let env = Environment::new();
        b.iter(|| {
            black_box(env.render_str(
                black_box(with_macros_template()),
                black_box(&serde_json::json!({"items": []})),
            ))
        });
    });

    group.finish();
}

fn compile_complex_templates(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_complex");
    group.sample_size(50);

    for complexity in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(complexity),
            complexity,
            |b, &n| {
                let env = Environment::new();
                // Generate a template with n levels of nesting
                let mut template = String::new();
                for i in 0..n {
                    template.push_str(&format!("{{% if condition_{} %}}", i));
                }
                template.push_str("{{ value }}");
                for _i in 0..n {
                    template.push_str("{% endif %}");
                }

                b.iter(|| {
                    black_box(env.render_str(
                        black_box(template.as_str()),
                        black_box(&serde_json::json!({"value": "test"})),
                    ))
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, compile_benchmarks, compile_complex_templates);
criterion_main!(benches);
