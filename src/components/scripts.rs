use maud::{Markup, PreEscaped, html};

pub fn scripts(active_story_id: Option<i64>) -> Markup {
    let active_id_str = active_story_id.map(|id| id.to_string()).unwrap_or_default();
    let js = format!(
        r#"
  const FEEDS = ['top', 'new', 'best', 'ask', 'show', 'jobs'];
  let currentActiveStoryId = "{active_id_str}";

  function setActiveFeed(feed) {{
    if (!feed || !FEEDS.includes(feed)) feed = 'top';
    FEEDS.forEach(f => {{
      const isTarget = f === feed;
      // Sync desktop tabs
      const dTab = document.getElementById(`tab-${{f}}`);
      if (dTab) {{
        dTab.setAttribute('aria-selected', isTarget ? 'true' : 'false');
        dTab.className = `nav-tab px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5 ${{isTarget ? 'bg-amber-500 text-neutral-950 font-semibold shadow' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60'}}`;
      }}
      // Sync mobile tabs
      const mTab = document.getElementById(`m-tab-${{f}}`);
      if (mTab) {{
        mTab.setAttribute('aria-selected', isTarget ? 'true' : 'false');
        mTab.className = `nav-tab flex-shrink-0 px-3 py-1.5 rounded-lg ${{isTarget ? 'bg-amber-500 text-neutral-950 font-bold' : 'text-neutral-400 bg-neutral-950 border border-neutral-800'}}`;
      }}
    }});
    const searchInput = document.getElementById('search-input');
    if (searchInput && feed) searchInput.value = '';
  }}

  function syncTabFromUrl() {{
    const params = new URLSearchParams(window.location.search);
    setActiveFeed(params.get('type') || 'top');
  }}

  function selectStoryCard(storyId) {{
    currentActiveStoryId = storyId;
    document.querySelectorAll('.story-card').forEach(c => c.classList.remove('active-story'));
    const target = document.getElementById(`story-card-${{storyId}}`);
    if (target) target.classList.add('active-story');
  }}

  function showMobileDetail() {{
    const layout = document.getElementById('app-layout');
    if (layout) {{
      layout.classList.remove('mobile-view-list');
      layout.classList.add('mobile-view-detail');
    }}
  }}

  function showMobileList() {{
    const layout = document.getElementById('app-layout');
    if (layout) {{
      layout.classList.remove('mobile-view-detail');
      layout.classList.add('mobile-view-list');
    }}
  }}

  function toggleAllComments(open) {{
    document.querySelectorAll('#comments-tree details').forEach(d => d.open = open);
  }}

  window.addEventListener('popstate', syncTabFromUrl);
  document.body.addEventListener('htmx:before:history:restore', syncTabFromUrl);

  // Progressive Loading Bar Controller for Item Navigation
  let progressTimerA = null;
  let progressTimerB = null;
  let progressResetTimer = null;

  function isDetailRequest(evt) {{
    const targetId = evt.detail?.ctx?.target?.id || evt.detail?.target?.id;
    const source = evt.detail?.ctx?.sourceElement || evt.target;
    return targetId === 'detail-pane' || (source && typeof source.closest === 'function' && source.closest('.story-card'));
  }}

  function startDetailProgress() {{
    const container = document.getElementById('detail-progress');
    const line = document.getElementById('detail-progress-line');
    if (!container || !line) return;

    clearTimeout(progressTimerA);
    clearTimeout(progressTimerB);
    clearTimeout(progressResetTimer);

    line.style.transition = 'none';
    line.style.width = '0%';
    container.classList.remove('opacity-0');
    container.classList.add('opacity-100');

    requestAnimationFrame(() => {{
      // Stage 1: Quick punch to 32%
      line.style.transition = 'width 220ms cubic-bezier(0.12, 0.85, 0.25, 1)';
      line.style.width = '32%';

      // Stage 2: Steady trickle to 72%
      progressTimerA = setTimeout(() => {{
        line.style.transition = 'width 550ms cubic-bezier(0.2, 0.8, 0.35, 1)';
        line.style.width = '72%';

        // Stage 3: Smooth creep to 90%
        progressTimerB = setTimeout(() => {{
          line.style.transition = 'width 1500ms ease-out';
          line.style.width = '90%';
        }}, 550);
      }}, 220);
    }});
  }}

  function finishDetailProgress() {{
    const container = document.getElementById('detail-progress');
    const line = document.getElementById('detail-progress-line');
    if (!container || !line) return;

    clearTimeout(progressTimerA);
    clearTimeout(progressTimerB);
    clearTimeout(progressResetTimer);

    // Snap to 100% with luminous flash
    line.style.transition = 'width 140ms cubic-bezier(0, 0, 0.2, 1)';
    line.style.width = '100%';

    // Smooth fade out and reset
    progressResetTimer = setTimeout(() => {{
      container.classList.remove('opacity-100');
      container.classList.add('opacity-0');
      progressResetTimer = setTimeout(() => {{
        line.style.transition = 'none';
        line.style.width = '0%';
      }}, 220);
    }}, 160);
  }}

  document.body.addEventListener('htmx:before:request', function(evt) {{
    if (isDetailRequest(evt)) {{
      startDetailProgress();
    }}
  }});

  document.body.addEventListener('htmx:after:request', function(evt) {{
    if (isDetailRequest(evt)) {{
      finishDetailProgress();
    }}
  }});

  document.body.addEventListener('htmx:after:swap', function(evt) {{
    const targetId = evt.detail?.ctx?.target?.id || evt.detail?.target?.id;
    if (targetId === 'detail-pane') {{
      const detailScroll = document.getElementById('story-detail-content');
      if (detailScroll) detailScroll.scrollTop = 0;
      if (currentActiveStoryId) selectStoryCard(currentActiveStoryId);
    }}
  }});

  // Keyboard Shortcuts
  window.addEventListener('keydown', (e) => {{
    if (['INPUT', 'TEXTAREA'].includes(document.activeElement?.tagName)) {{
      if (e.key === 'Escape') document.activeElement.blur();
      return;
    }}
    const cards = Array.from(document.querySelectorAll('.story-card'));
    if (cards.length === 0) return;
    const curIdx = cards.findIndex(c => c.id === `story-card-${{currentActiveStoryId}}`);

    if (e.key === 'j' || e.key === 'ArrowDown') {{
      e.preventDefault();
      const next = cards[curIdx < cards.length - 1 ? curIdx + 1 : 0];
      if (next) {{ next.click(); next.scrollIntoView({{ block: 'nearest', behavior: 'smooth' }}); }}
    }} else if (e.key === 'k' || e.key === 'ArrowUp') {{
      e.preventDefault();
      const prev = cards[curIdx > 0 ? curIdx - 1 : cards.length - 1];
      if (prev) {{ prev.click(); prev.scrollIntoView({{ block: 'nearest', behavior: 'smooth' }}); }}
    }} else if (e.key === '/') {{
      e.preventDefault();
      const s = document.getElementById('search-input');
      if (s) {{ s.focus(); s.select(); }}
    }} else if (e.key === 'c') {{
      const first = document.querySelector('#comments-tree details');
      if (first) toggleAllComments(!first.open);
    }} else if (e.key === 'o') {{
      const active = document.getElementById(`story-card-${{currentActiveStoryId}}`);
      const link = active?.querySelector('a[target="_blank"]');
      if (link) window.open(link.href, '_blank');
    }}
  }});

  window.addEventListener('DOMContentLoaded', () => {{
    if (currentActiveStoryId) selectStoryCard(currentActiveStoryId);
  }});
"#
    );

    html! {
        script {
            (PreEscaped(js))
        }
    }
}
