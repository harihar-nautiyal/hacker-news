---
name: maud-htmx-spa
description: Comprehensive guide, architectural patterns, and syntax reference for building hypermedia Single Page Applications (SPAs) with Maud (type-safe Rust HTML templates), HTMX, Tailwind CSS, and Actix Web.
---

# Maud + HTMX SPA Development in Rust

This guide outlines architectural patterns, syntax conventions, and best practices for building high-performance, type-safe hypermedia applications using **Maud**, **HTMX**, **Actix Web**, and **Tailwind CSS**.

---

## 1. Maud Templating Fundamentals

Maud compiles HTML templates directly into Rust code at compile-time with zero runtime overhead and compile-time HTML verification.

### Core Syntax Rules

1. **Elements & Blocks**:
   ```rust
   html! {
       div class="container mx-auto p-4" {
           h1 class="text-xl font-bold" { "Hello, world!" }
           p { "Welcome to Maud." }
       }
   }
   ```

2. **Self-Closing / Void Elements**:
   Self-closing elements (`meta`, `link`, `input`, `br`, `hr`, `img`) end with a semicolon `;`:
   ```rust
   html! {
       meta charset="UTF-8";
       meta name="viewport" content="width=device-width, initial-scale=1.0";
       link rel="stylesheet" href="/static/tailwind.css";
       input type="text" name="query" placeholder="Search...";
   }
   ```

3. **Dynamic Variables & Interpolation**:
   Wrap Rust expressions in parentheses `(...)`:
   ```rust
   let username = "alice";
   let score = 42;

   html! {
       span { (username) }
       span class="score" { (score) }
       a href=(format!("/user/{}", username)) { "Profile" }
   }
   ```

4. **Raw / Unescaped HTML**:
   Maud escapes all strings by default. To render raw HTML (e.g. pre-sanitized markdown or user content):
   ```rust
   use maud::PreEscaped;

   let safe_html = "<p><strong>Formatted</strong> text</p>";
   html! {
       div class="prose" {
           (PreEscaped(safe_html))
       }
   }
   ```

5. **Conditionals (`@if`, `@else if`, `@else`)**:
   ```rust
   html! {
       @if let Some(detail) = &self.active_detail {
           (story_detail(detail))
       } @else {
           (empty_detail())
       }

       @if count > 0 {
           span { (count) " items" }
       } @else {
           span { "No items" }
       }
   }
   ```

6. **Loops (`@for`, `@while`)**:
   ```rust
   html! {
       ul class="space-y-2" {
           @for item in &items {
               li id=(format!("item-{}", item.id)) {
                   (item.title)
               }
           }
       }
   }
   ```

7. **Local Variables (`@let`)**:
   ```rust
   html! {
       @for feed in FeedType::ALL {
           @let is_active = active_feed == feed;
           button class=(if is_active { "bg-amber-500 font-bold" } else { "text-neutral-400" }) {
               (feed.label())
           }
       }
   }
   ```

### ⚠️ Common Maud Pitfalls

- **`class=[...]` array syntax is invalid**: Maud does NOT support class arrays. Use Rust conditional expressions:
  - ❌ `class=["btn", if active { "btn-active" }]`
  - ✅ `class=(if active { "btn btn-active" } else { "btn" })`
- **DOCTYPE declaration**:
  ```rust
  use maud::DOCTYPE;

  html! {
      (DOCTYPE)
      html lang="en" { ... }
  }
  ```

---

## 2. Component & Route Architecture (TypedBuilder Pattern)

Structure views and components as type-safe Rust structs using `typed-builder` to enforce strict contracts and enable clean composition:

```rust
use maud::{Markup, html};
use typed_builder::TypedBuilder;
use crate::models::{FeedType, StorySummary};

#[derive(Debug, Clone, TypedBuilder)]
pub struct StoriesList<'a> {
    pub category_icon: &'a str,
    pub category_title: &'a str,
    pub stories: &'a [StorySummary],
    pub search_query: &'a str,
    pub feed_type: FeedType,
    pub next_page: u32,
}

impl<'a> StoriesList<'a> {
    pub fn render(&self) -> Markup {
        html! {
            div class="flex flex-col h-full overflow-hidden" {
                // Header Partial
                (feed_header(self.category_icon, self.category_title, self.stories, self.search_query, self.feed_type))

                // Scroll Container
                div id="stories-scroll-container" class="flex-1 overflow-y-auto p-3 space-y-2.5" {
                    (stories_items(self.stories))

                    @if self.stories.is_empty() {
                        (empty_stories(self.search_query))
                    } @else {
                        (load_more(self.feed_type, self.next_page, self.search_query))
                    }
                }
            }
        }
    }
}

// Convenient function wrapper
pub fn stories_list(
    category_icon: &str,
    category_title: &str,
    stories: &[StorySummary],
    search_query: &str,
    feed_type: FeedType,
    next_page: u32,
) -> Markup {
    StoriesList::builder()
        .category_icon(category_icon)
        .category_title(category_title)
        .stories(stories)
        .search_query(search_query)
        .feed_type(feed_type)
        .next_page(next_page)
        .build()
        .render()
}
```

---

## 3. Actix Web + Maud Route Handling

Enable the `actix-web` feature in `Cargo.toml`:
```toml
maud = { version = "0.27.0", features = ["actix-web"] }
```

With this feature, `maud::Markup` directly implements `actix_web::Responder` with `Content-Type: text/html; charset=utf-8`.

### Differentiating HTMX Partials vs Full-Page Browser Navigations

```rust
use actix_web::{HttpRequest, HttpResponse, Responder, get, web::{Data, Path, Query}};
use maud::{Markup, html};

#[get("/item/{id}")]
pub async fn get_item_detail(
    req: HttpRequest,
    service: Data<AppState>,
    path: Path<i64>,
    query: Query<ItemQuery>,
) -> impl Responder {
    let id = path.into_inner();
    let detail = match service.get_item(id, query.refresh.unwrap_or(false)).await {
        Ok(d) => d,
        Err(err) => {
            return HttpResponse::NotFound()
                .content_type("text/html; charset=utf-8")
                .body(story_not_found_markup(&err).into_string());
        }
    };

    let is_htmx = req.headers().contains_key("hx-request");

    if is_htmx {
        // Return partial HTML directly for HTMX swaps into #detail-pane
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(story_detail(&detail).into_string())
    } else {
        // Direct browser visit: render full page layout with sidebar + active detail
        let stories = service.get_feed(FeedType::Top, 0, false).await.unwrap_or_default();
        let index_page = Index::builder()
            .title(format!("{} | Hacker News", detail.title))
            .active_feed(FeedType::Top)
            .has_active_detail(true)
            .category_icon("🔥".to_string())
            .category_title("Top Stories".to_string())
            .feed_type(FeedType::Top)
            .search_query(String::new())
            .stories(stories)
            .next_page(1)
            .active_detail(Some(detail))
            .build();

        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(index_page.render().into_string())
    }
}
```

---

## 4. HTMX SPA Patterns & Indicators

### 1. Item Navigation & Progressive Loading Bar
Target the detail pane with `hx-target="#detail-pane"` and assign an indicator:
```rust
article
    id=(format!("story-card-{}", story.id))
    class="story-card ..."
    hx-get=(format!("/item/{}", story.id))
    hx-target="#detail-pane"
    hx-push-url=(format!("/item/{}", story.id))
    hx-swap="innerHTML"
    hx-indicator="#detail-progress"
    hx-preload="mouseover"
    onclick=(format!("selectStoryCard('{}'); showMobileDetail();", story.id))
{ ... }
```

### 2. Supporting Both Colon and CamelCase HTMX Event Names
Certain HTMX builds dispatch colon-separated custom events (`htmx:before:request`, `htmx:after:request`, `htmx:after:swap`) while others use camelCase (`htmx:beforeRequest`). Always bind both in client scripts:

```javascript
const beforeRequestEvents = ['htmx:before:request', 'htmx:beforeRequest', 'htmx:config:request'];
const afterRequestEvents = ['htmx:after:request', 'htmx:afterRequest', 'htmx:finally:request'];

beforeRequestEvents.forEach((evtName) => {
  document.addEventListener(evtName, (evt) => {
    if (isDetailRequest(evt)) startDetailProgress();
  });
});

afterRequestEvents.forEach((evtName) => {
  document.addEventListener(evtName, (evt) => {
    if (isDetailRequest(evt)) finishDetailProgress();
  });
});
```

### 3. Progressive Luminous Indicator Animation

CSS:
```css
#detail-progress {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  z-index: 40;
  pointer-events: none;
  background: rgba(245, 158, 11, 0.08);
  overflow: hidden;
}

#detail-progress-line {
  height: 100%;
  background: linear-gradient(90deg, #d97706 0%, #f59e0b 45%, #fbbf24 85%, #fffbeb 100%);
  box-shadow: 0 0 14px rgba(245, 158, 11, 0.85), 0 0 6px rgba(251, 191, 36, 0.9);
  position: relative;
  border-radius: 0 9999px 9999px 0;
}

#detail-progress-line::after {
  content: '';
  position: absolute;
  top: -2px;
  right: 0;
  bottom: -2px;
  width: 60px;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.95) 85%, #ffffff);
  filter: drop-shadow(0 0 6px #fbbf24) drop-shadow(0 0 10px #f59e0b);
  border-radius: 0 9999px 9999px 0;
}
```

JS Controller:
```javascript
function startDetailProgress() {
  const container = document.getElementById('detail-progress');
  const line = document.getElementById('detail-progress-line');
  if (!container || !line) return;

  line.style.transition = 'none';
  line.style.width = '0%';
  container.classList.remove('opacity-0');
  container.classList.add('opacity-100');

  requestAnimationFrame(() => {
    // Stage 1: Quick punch to 35%
    line.style.transition = 'width 200ms cubic-bezier(0.12, 0.85, 0.25, 1)';
    line.style.width = '35%';

    // Stage 2: Steady trickle to 75%
    setTimeout(() => {
      line.style.transition = 'width 500ms cubic-bezier(0.2, 0.8, 0.35, 1)';
      line.style.width = '75%';
    }, 200);
  });
}

function finishDetailProgress() {
  const container = document.getElementById('detail-progress');
  const line = document.getElementById('detail-progress-line');
  if (!container || !line) return;

  line.style.transition = 'width 150ms cubic-bezier(0, 0, 0.2, 1)';
  line.style.width = '100%';

  setTimeout(() => {
    container.classList.remove('opacity-100');
    container.classList.add('opacity-0');
  }, 250);
}
```

---

## 5. Embedded Asset Management (`RustEmbed`) & `build.rs`

Embed production assets into the binary while maintaining automatic rebuilds:

```rust
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
pub struct Static;

#[get("/static/{filename:.*}")]
async fn serve_assets(path: actix_web::web::Path<String>) -> impl actix_web::Responder {
    let filename = path.into_inner();
    match Static::get(&filename) {
        Some(file) => actix_web::HttpResponse::Ok()
            .content_type(mime_guess::from_path(&filename).first_or_octet_stream().as_ref())
            .body(file.data.into_owned()),
        None => actix_web::HttpResponse::NotFound().body("404 Not Found"),
    }
}
```

In `build.rs`, rerun compilation when CSS inputs or source templates change:
```rust
println!("cargo:rerun-if-changed=src");
println!("cargo:rerun-if-changed=src/input.css");
```

---

## 6. Checklist for Porting Askama/Jinja to Maud

1. **Meta / Void Tags**: Convert `<meta ...>` / `<link ...>` to `meta ...;` and `link ...;`.
2. **Dynamic Class Expressions**: Convert array syntax `["base", condition && "active"]` into `(if condition { "base active" } else { "base" })`.
3. **PreEscaped**: Wrap HTML strings from databases/APIs in `PreEscaped(...)`.
4. **TypedBuilder Models**: Define typed view structs for all multi-parameter components.
5. **Separate Scripts Directory**: Place client scripts in `static/scripts/` and reference via `<script src="/static/scripts/app.js" defer></script>`.
6. **Dual Event Listeners**: Support both colon and camelCase HTMX event names.
