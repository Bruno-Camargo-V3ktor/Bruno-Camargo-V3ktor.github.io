(() => {
  window.blueRoseMotion?.();
  if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  const observed = new WeakSet();
  const observer = new IntersectionObserver(entries => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        entry.target.classList.add('is-revealed');
        observer.unobserve(entry.target);
      }
    });
  }, {threshold: .12});
  const discover = () => {
    document.querySelectorAll('.home-flow .section-heading, .home-flow .content-card, .home-flow .home-about > div, .home-flow .home-certifications, .home-flow .home-contact, .certificate-card').forEach(el => {
      if (!observed.has(el)) {
        observed.add(el);
        el.classList.add('scroll-reveal');
        observer.observe(el);
      }
    });
  };
  let frame;
  const scroll = () => {
    if (frame) return;
    frame = requestAnimationFrame(() => {
      frame = null;
      document.querySelectorAll('.home-flow > .section').forEach(el => {
        const progress = Math.max(-1, Math.min(1, el.getBoundingClientRect().top / innerHeight));
        el.style.setProperty('--light-shift', `${progress * 70}px`);
      });
    });
  };
  const mutations = new MutationObserver(discover);
  mutations.observe(document.getElementById('main'), {childList:true, subtree:true});
  discover(); scroll();
  addEventListener('scroll', scroll, {passive:true});
  window.blueRoseMotion = () => {
    observer.disconnect(); mutations.disconnect();
    removeEventListener('scroll', scroll); cancelAnimationFrame(frame);
  };
})();
