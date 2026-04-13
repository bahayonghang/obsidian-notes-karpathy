import DefaultTheme from 'vitepress/theme'
import WorkflowLifecycleDiagram from './components/WorkflowLifecycleDiagram.vue'
import './custom.css'

export default {
  ...DefaultTheme,
  enhanceApp({ app }) {
    DefaultTheme.enhanceApp?.({ app })
    app.component('WorkflowLifecycleDiagram', WorkflowLifecycleDiagram)
  },
}
