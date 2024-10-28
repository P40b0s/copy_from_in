import 'highlight.js/styles/github-dark-dimmed.css'
import hljs from 'highlight.js/lib/core';
import xml from 'highlight.js/lib/languages/xml';
import hljsVuePlugin from "@highlightjs/vue-plugin";

hljs.registerLanguage('xml', xml);
export {hljs, hljsVuePlugin}