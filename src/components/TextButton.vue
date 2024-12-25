<template>
  <div class="btn" :class="[{ disabled: props.disabled }, colorClass]"><slot></slot></div>
</template>

<script setup lang="ts">
import { computed, PropType } from "vue";

const props = defineProps({
  disabled: {
    type: Boolean,
    default: false,
  },
  type: {
    type: String as PropType<ButtonType>,
    default: "primary",
  },
});

const colorClass = computed(() => {
  switch (props.type) {
    case "primary":
      return "primary";
    case "success":
      return "success";
    case "warning":
      return "warning";
    case "danger":
      return "danger";
    case "info":
      return "info";
    default:
      return "primary";
  }
});
</script>

<style lang="scss" scoped>
.btn {
  cursor: pointer;
  height: 100%;
  font-size: 14px;
}

.primary {
  color: #409eff;
  &:hover {
    color: rgb(159.5, 206.5, 255);
  }
}
.success {
  color: #67c23a;
  &:hover {
    color: rgb(179, 224.5, 156.5);
  }
}
.warning {
  color: #e6a23c;
  &:hover {
    color: rgb(242.5, 208.5, 157.5);
  }
}
.danger {
  color: #f56c6c;
  &:hover {
    color: rgb(250, 181.5, 181.5);
  }
}
.info {
  color: #909399;
  &:hover {
    color: rgb(199.5, 201, 204);
  }
}

.disabled {
  cursor: not-allowed !important;
  color: #ccc;
}
</style>
