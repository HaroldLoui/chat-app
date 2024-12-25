<template>
  <vs-dialog v-model="open" width="550px" not-center>
    <template #header>
      <h6 class="not-margin"></h6>
    </template>

    <div class="con-content">
      <p>{{ message }}</p>
    </div>

    <template #footer>
      <div class="con-footer">
        <vs-button type="transparent" @click="confirm"> 确认 </vs-button>
        <vs-button color="danger" type="transparent" @click="open = false"> 取消 </vs-button>
      </div>
    </template>
  </vs-dialog>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const open = ref<boolean>(false);
const message = ref<string>("");

const emits = defineEmits(["confirm"]);
const confirm = () => {
  emits("confirm");
};
const show = (msg: string) => {
  message.value = msg;
  open.value = true;
};
const close = () => {
  open.value = false;
};
defineExpose({ show, close });
</script>

<style lang="scss" scoped>
.not-margin {
  margin: 0px;
  font-weight: normal;
  padding: 10px;
}

.con-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  width: calc(100%);
  .new {
    margin: 0px;
    margin-top: 20px;
    padding: 0px;
    font-size: 0.7rem;
    a {
      color: rgba(var(--vs-primary)) !important;
      margin-left: 6px;
      &:hover {
        text-decoration: underline;
      }
    }
  }
  .vs-button {
    margin: 0px;
  }
}
</style>
