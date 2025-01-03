<template>
  <vs-alert v-model="active" v-model:hidden-content="hidden" :progress="progress" :color="color" closable>
    <template #title> {{ desc }} </template>
  </vs-alert>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";

const active = ref<boolean>(false);
const hidden = ref<boolean>(true);
const time = ref<number>(6000);
const progress = ref<number>(0);

const props = defineProps({
  type: {
    type: Boolean,
    required: true,
  },
});

const color = computed(() => (props.type ? "success" : "danger"));
const desc = computed(() => (props.type ? "设置代理成功" : "设置代理失败"));

watch(active, (val: boolean) => {
  if (val) {
    const interval = setInterval(() => {
      progress.value++;
    }, time.value / 100);

    setTimeout(() => {
      active.value = false;
      clearInterval(interval);
      progress.value = 0;
    }, time.value);
  }
});

const show = () => {
  active.value = true;
};

defineExpose({ show });
</script>
