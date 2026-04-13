<script setup>
import { computed } from 'vue'

const props = defineProps({
  locale: {
    type: String,
    default: 'en',
  },
})

const copy = computed(() => {
  if (props.locale === 'zh') {
    return {
      title: '知识库生命周期流程图',
      description:
        '从 obsidian-notes-karpathy 入口路由到 kb-init、kb-ingest、kb-compile、kb-review、kb-query 和 kb-render，并显示回写与维护回路。',
      labels: {
        writeback: '回写候选需重新进入草稿',
        durableKnowledge: '新的长期知识回流草稿层',
        maintenance: '维护模式在已批准层内修复或重建',
      },
      legend: '实线表示常规生命周期，虚线表示回写与维护回路。',
    }
  }

  return {
    title: 'Knowledge base lifecycle diagram',
    description:
      'Route from the obsidian-notes-karpathy entry skill into kb-init, kb-ingest, kb-compile, kb-review, kb-query, and kb-render, with writeback and maintenance loops.',
    labels: {
      writeback: 'Writeback candidates return to drafts',
      durableKnowledge: 'New durable knowledge returns to drafts',
      maintenance: 'Maintenance mode repairs or rebuilds in place',
    },
    legend: 'Solid arrows show the normal lifecycle. Dashed arrows show writeback and maintenance loops.',
  }
})

const idBase = computed(() => `workflow-lifecycle-${props.locale}`)
const arrowMarker = computed(() => `url(#${idBase.value}-arrow-head)`)

const nodes = [
  { key: 'entry', label: 'obsidian-notes-karpathy', x: 40, y: 160, width: 220, height: 56, root: true },
  { key: 'init', label: 'kb-init', x: 340, y: 20, width: 170, height: 48 },
  { key: 'ingest', label: 'kb-ingest', x: 340, y: 95, width: 170, height: 48 },
  { key: 'compile', label: 'kb-compile', x: 340, y: 170, width: 170, height: 48 },
  { key: 'review', label: 'kb-review', x: 340, y: 245, width: 170, height: 48 },
  { key: 'query', label: 'kb-query', x: 640, y: 140, width: 170, height: 48 },
  { key: 'render', label: 'kb-render', x: 640, y: 240, width: 170, height: 48 },
]
</script>

<template>
  <figure class="workflow-diagram">
    <svg
      viewBox="0 0 860 380"
      role="img"
      :aria-labelledby="`${idBase}-title ${idBase}-desc`"
    >
      <title :id="`${idBase}-title`">{{ copy.title }}</title>
      <desc :id="`${idBase}-desc`">{{ copy.description }}</desc>

      <defs>
        <marker
          :id="`${idBase}-arrow-head`"
          viewBox="0 0 10 10"
          refX="8"
          refY="5"
          markerWidth="7"
          markerHeight="7"
          orient="auto-start-reverse"
        >
          <path d="M 0 0 L 10 5 L 0 10 z" class="workflow-diagram__marker" />
        </marker>
      </defs>

      <g class="workflow-diagram__routes">
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M260 188 H300" />
        <path class="workflow-diagram__edge" d="M300 44 V269" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M300 44 H340" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M300 119 H340" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M300 194 H340" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M300 269 H340" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M260 188 H600 V164 H640" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M260 188 H600 V264 H640" />
      </g>

      <g class="workflow-diagram__lifecycle">
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M425 68 V95" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M425 143 V170" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M425 218 V245" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M510 269 H575 V164 H640" />
        <path class="workflow-diagram__edge" :marker-end="arrowMarker" d="M510 269 H575 V264 H640" />
      </g>

      <g class="workflow-diagram__feedback">
        <path
          class="workflow-diagram__edge workflow-diagram__edge--feedback"
          :marker-end="arrowMarker"
          d="M640 164 H785 V325 H425 V218"
        />
        <text class="workflow-diagram__label" x="786" y="208">{{ copy.labels.writeback }}</text>

        <path
          class="workflow-diagram__edge workflow-diagram__edge--feedback"
          :marker-end="arrowMarker"
          d="M640 264 H735 V308 H425 V218"
        />
        <text class="workflow-diagram__label" x="646" y="326">{{ copy.labels.durableKnowledge }}</text>

        <path
          class="workflow-diagram__edge workflow-diagram__edge--feedback"
          :marker-end="arrowMarker"
          d="M510 269 C575 269 575 323 510 323 C540 323 540 269 510 269"
        />
        <text class="workflow-diagram__label" x="534" y="344">{{ copy.labels.maintenance }}</text>
      </g>

      <g v-for="node in nodes" :key="node.key" class="workflow-diagram__node" :class="{ 'workflow-diagram__node--root': node.root }">
        <rect :x="node.x" :y="node.y" :width="node.width" :height="node.height" rx="14" ry="14" />
        <text :x="node.x + node.width / 2" :y="node.y + node.height / 2 + 1">{{ node.label }}</text>
      </g>
    </svg>
    <figcaption class="workflow-diagram__legend">{{ copy.legend }}</figcaption>
  </figure>
</template>
