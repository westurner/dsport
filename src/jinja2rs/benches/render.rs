use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use jinja2rs::environment::Environment;
use serde_json::json;

// Benchmark fixtures with varying complexity

/// Simple template: basic variable interpolation
fn simple_template() -> &'static str {
    "Hello {{ name }}!"
}

/// Medium template: loops and conditionals
fn medium_template() -> &'static str {
    r#"
{% for item in items %}
  {% if item.active %}
    <li class="active">{{ item.name }}</li>
  {% else %}
    <li>{{ item.name }}</li>
  {% endif %}
{% endfor %}
"#
}

/// Complex template: nested loops, filters, multiple globals
fn complex_template() -> &'static str {
    r#"
<div class="results">
  {% for category in categories %}
    <section>
      <h2>{{ category.title }}</h2>
      <ul>
        {% for item in category.items %}
          <li data-id="{{ item.id }}">
            {{ item.name | upper }}
            {% if item.description %}
              <p>{{ item.description | wordwrap(60) }}</p>
            {% endif %}
          </li>
        {% endfor %}
      </ul>
    </section>
  {% endfor %}
</div>
"#
}

fn render_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");

    // Simple render
    group.bench_function("simple", |b| {
        let env = Environment::new();
        let ctx = json!({"name": "World"});
        b.iter(|| black_box(env.render_str(black_box(simple_template()), black_box(&ctx))));
    });

    // Medium complexity render
    group.bench_function("medium", |b| {
        let env = Environment::new();
        let ctx = json!({
            "items": [
                {"name": "Item 1", "active": true},
                {"name": "Item 2", "active": false},
                {"name": "Item 3", "active": true},
            ]
        });
        b.iter(|| black_box(env.render_str(black_box(medium_template()), black_box(&ctx))));
    });

    // Complex render with real data
    group.bench_function("complex", |b| {
        let env = Environment::new();
        let ctx = json!({
            "categories": [
                {
                    "title": "Category A",
                    "items": [
                        {"id": 1, "name": "Item 1", "description": "A sample description for the first item"},
                        {"id": 2, "name": "Item 2", "description": "Another description"},
                        {"id": 3, "name": "Item 3", "description": null},
                    ]
                },
                {
                    "title": "Category B",
                    "items": [
                        {"id": 4, "name": "Item 4", "description": "Description for category B"},
                        {"id": 5, "name": "Item 5", "description": null},
                    ]
                },
            ]
        });
        b.iter(|| {
            black_box(env.render_str(black_box(complex_template()), black_box(&ctx)))
        });
    });

    // Render with filter chains
    group.bench_function("filter_chain", |b| {
        let env = Environment::new();
        let ctx = json!({"text": "hello world this is a test message for filtering"});
        let template = "{{ text | upper | wordwrap(10) }}";
        b.iter(|| black_box(env.render_str(black_box(template), black_box(&ctx))));
    });

    // Render with nested data access
    group.bench_function("deep_access", |b| {
        let env = Environment::new();
        let ctx = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "value": "deeply nested"
                    }
                }
            }
        });
        let template = "{{ level1.level2.level3.value }}";
        b.iter(|| black_box(env.render_str(black_box(template), black_box(&ctx))));
    });

    group.finish();
}

fn render_many_iterations(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_iterations");
    group.sample_size(100);

    for item_count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(item_count),
            item_count,
            |b, &count| {
                let env = Environment::new();
                let items: Vec<_> = (0..count)
                    .map(|i| json!({"id": i, "name": format!("Item {}", i), "active": i % 2 == 0}))
                    .collect();
                let ctx = json!({"items": items});
                let template = r#"
{% for item in items %}
  [{{ item.id }}] {{ item.name }} {% if item.active %}ACTIVE{% endif %}
{% endfor %}
"#;
                b.iter(|| black_box(env.render_str(black_box(template), black_box(&ctx))));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, render_benchmarks, render_many_iterations);
criterion_main!(benches);
