console.log("CONFIG FILE LOADED");

window.MathJax = {
  tex: {
    inlineMath: [["$", "$"], ["\\(", "\\)"]],
    displayMath: [["$$", "$$"], ["\\[", "\\]"]],
  },
    chtml: {
    scale: 1.0,
    minScale: 0.5,
    matchFontHeight: false,
  },
};



window.renderMath = () => {
  console.log("renderMath called");
  if (!window.MathJax || !window.MathJax.typesetPromise) return;

  MathJax.typesetPromise().then(() => {
    document.querySelectorAll('.block-math mjx-container').forEach(el => {
      // Reset previous scale so we remeasure correctly
      el.style.transform = "";

      const parent = el.parentElement;
      const parentWidth = parent.clientWidth;
      const contentWidth = el.scrollWidth;

      if (contentWidth > parentWidth) {
        const scale = parentWidth / contentWidth;
        console.log("Shrinking equation: scale =", scale);
        el.style.transform = `scale(${scale})`;
      }
    });
  });
};
