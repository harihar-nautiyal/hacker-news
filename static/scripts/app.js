/**
 * Hacker News HTMX SPA - Client Application Script
 */

const FEEDS = ['top', 'new', 'best', 'ask', 'show', 'jobs'];
let currentActiveStoryId = null;

function getInitialActiveStoryId() {
  const meta = document.querySelector('meta[name="active-story-id"]');
  if (meta && meta.content) return meta.content;
  const layout = document.getElementById('app-layout');
  if (layout && layout.dataset.activeStoryId) return layout.dataset.activeStoryId;
  return null;
}

function setActiveFeed(feed) {
  if (!feed || !FEEDS.includes(feed)) feed = 'top';
  FEEDS.forEach((f) => {
    const isTarget = f === feed;
    // Sync desktop tabs
    const dTab = document.getElementById(`tab-${f}`);
    if (dTab) {
      dTab.setAttribute('aria-selected', isTarget ? 'true' : 'false');
      dTab.className = `nav-tab px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5 ${
        isTarget
          ? 'bg-amber-500 text-neutral-950 font-semibold shadow'
          : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60'
      }`;
    }
    // Sync mobile tabs
    const mTab = document.getElementById(`m-tab-${f}`);
    if (mTab) {
      mTab.setAttribute('aria-selected', isTarget ? 'true' : 'false');
      mTab.className = `nav-tab flex-shrink-0 px-3 py-1.5 rounded-lg ${
        isTarget
          ? 'bg-amber-500 text-neutral-950 font-bold'
          : 'text-neutral-400 bg-neutral-950 border border-neutral-800'
      }`;
    }
  });
  const searchInput = document.getElementById('search-input');
  if (searchInput && feed) searchInput.value = '';
}

function syncTabFromUrl() {
  const params = new URLSearchParams(window.location.search);
  setActiveFeed(params.get('type') || 'top');
}

function selectStoryCard(storyId) {
  if (!storyId) return;
  currentActiveStoryId = storyId;
  document.querySelectorAll('.story-card').forEach((c) => c.classList.remove('active-story'));
  const target = document.getElementById(`story-card-${storyId}`);
  if (target) target.classList.add('active-story');
}

function showMobileDetail() {
  const layout = document.getElementById('app-layout');
  if (layout) {
    layout.classList.remove('mobile-view-list');
    layout.classList.add('mobile-view-detail');
  }
}

function showMobileList() {
  const layout = document.getElementById('app-layout');
  if (layout) {
    layout.classList.remove('mobile-view-detail');
    layout.classList.add('mobile-view-list');
  }
}

function toggleAllComments(open) {
  document.querySelectorAll('#comments-tree details').forEach((d) => (d.open = open));
}

// ----------------------------------------------------
// Item Detail Shell Top Progress Bar (#detail-progress)
// ----------------------------------------------------
let progressTimerA = null;
let progressTimerB = null;
let progressResetTimer = null;
let detailStartTime = 0;

function isDetailRequest(evt) {
  const ctx = evt.detail?.ctx;
  const target = ctx?.target || evt.detail?.target;
  const targetId = target?.id || evt.detail?.elt?.getAttribute('hx-target')?.replace('#', '');
  const source = ctx?.sourceElement || evt.detail?.elt || evt.target;
  return (
    targetId === 'detail-pane' ||
    (source && typeof source.closest === 'function' && !!source.closest('.story-card'))
  );
}

function startDetailProgress() {
  const container = document.getElementById('detail-progress');
  const line = document.getElementById('detail-progress-line');
  if (!container || !line) return;

  clearTimeout(progressTimerA);
  clearTimeout(progressTimerB);
  clearTimeout(progressResetTimer);
  detailStartTime = Date.now();

  line.style.transition = 'none';
  line.style.width = '0%';
  container.classList.remove('opacity-0');
  container.classList.add('opacity-100');

  requestAnimationFrame(() => {
    // Stage 1: Quick punch to 35%
    line.style.transition = 'width 200ms cubic-bezier(0.12, 0.85, 0.25, 1)';
    line.style.width = '35%';

    // Stage 2: Steady trickle to 75%
    progressTimerA = setTimeout(() => {
      line.style.transition = 'width 500ms cubic-bezier(0.2, 0.8, 0.35, 1)';
      line.style.width = '75%';

      // Stage 3: Smooth creep to 92%
      progressTimerB = setTimeout(() => {
        line.style.transition = 'width 1200ms ease-out';
        line.style.width = '92%';
      }, 500);
    }, 200);
  });
}

function finishDetailProgress() {
  const container = document.getElementById('detail-progress');
  const line = document.getElementById('detail-progress-line');
  if (!container || !line) return;

  clearTimeout(progressTimerA);
  clearTimeout(progressTimerB);
  clearTimeout(progressResetTimer);

  const elapsed = Date.now() - detailStartTime;
  const minTime = 250;
  const delay = Math.max(0, minTime - elapsed);

  setTimeout(() => {
    // Snap to 100% with luminous flash
    line.style.transition = 'width 150ms cubic-bezier(0, 0, 0.2, 1)';
    line.style.width = '100%';

    // Smooth fade out and reset
    progressResetTimer = setTimeout(() => {
      container.classList.remove('opacity-100');
      container.classList.add('opacity-0');
      progressResetTimer = setTimeout(() => {
        line.style.transition = 'none';
        line.style.width = '0%';
      }, 200);
    }, 180);
  }, delay);
}

// ----------------------------------------------------
// HTMX Event Listeners
// ----------------------------------------------------
const beforeRequestEvents = ['htmx:before:request', 'htmx:beforeRequest', 'htmx:config:request', 'htmx:configRequest'];
const afterRequestEvents = ['htmx:after:request', 'htmx:afterRequest', 'htmx:finally:request'];
const afterSwapEvents = ['htmx:after:swap', 'htmx:afterSwap'];
const historyEvents = ['htmx:before:history:restore', 'htmx:historyRestore', 'popstate'];

beforeRequestEvents.forEach((evtName) => {
  document.addEventListener(evtName, (evt) => {
    if (isDetailRequest(evt)) {
      startDetailProgress();
    }
  });
});

afterRequestEvents.forEach((evtName) => {
  document.addEventListener(evtName, (evt) => {
    if (isDetailRequest(evt)) {
      finishDetailProgress();
    }
  });
});

afterSwapEvents.forEach((evtName) => {
  document.addEventListener(evtName, (evt) => {
    const ctx = evt.detail?.ctx;
    const target = ctx?.target || evt.detail?.target;
    const targetId = target?.id;
    if (targetId === 'detail-pane') {
      const detailScroll = document.getElementById('story-detail-content');
      if (detailScroll) detailScroll.scrollTop = 0;
      if (currentActiveStoryId) selectStoryCard(currentActiveStoryId);
    }
    syncTabFromUrl();
  });
});

historyEvents.forEach((evtName) => {
  window.addEventListener(evtName, syncTabFromUrl);
  document.addEventListener(evtName, syncTabFromUrl);
});

// ----------------------------------------------------
// Keyboard Shortcuts
// ----------------------------------------------------
window.addEventListener('keydown', (e) => {
  if (['INPUT', 'TEXTAREA'].includes(document.activeElement?.tagName)) {
    if (e.key === 'Escape') document.activeElement.blur();
    return;
  }
  const cards = Array.from(document.querySelectorAll('.story-card'));
  if (cards.length === 0) return;
  const curIdx = cards.findIndex((c) => c.id === `story-card-${currentActiveStoryId}`);

  if (e.key === 'j' || e.key === 'ArrowDown') {
    e.preventDefault();
    const next = cards[curIdx < cards.length - 1 ? curIdx + 1 : 0];
    if (next) {
      next.click();
      next.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    }
  } else if (e.key === 'k' || e.key === 'ArrowUp') {
    e.preventDefault();
    const prev = cards[curIdx > 0 ? curIdx - 1 : cards.length - 1];
    if (prev) {
      prev.click();
      prev.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    }
  } else if (e.key === '/') {
    e.preventDefault();
    const s = document.getElementById('search-input');
    if (s) {
      s.focus();
      s.select();
    }
  } else if (e.key === 'c') {
    const first = document.querySelector('#comments-tree details');
    if (first) toggleAllComments(!first.open);
  } else if (e.key === 'o') {
    const active = document.getElementById(`story-card-${currentActiveStoryId}`);
    const link = active?.querySelector('a[target="_blank"]');
    if (link) window.open(link.href, '_blank');
  }
});

// ----------------------------------------------------
// Initialization
// ----------------------------------------------------
window.addEventListener('DOMContentLoaded', () => {
  currentActiveStoryId = getInitialActiveStoryId();
  if (currentActiveStoryId) {
    selectStoryCard(currentActiveStoryId);
  }
  syncTabFromUrl();
});
