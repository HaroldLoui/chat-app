<template>
  <div class="chat-box" :class="{ active: props.active }">
    <div class="title">{{ value.title }}</div>
    <div class="sub-title">
      <div class="count">共{{ value.count }}条对话</div>
      <div class="time">{{ value.createTime }}</div>
    </div>
    <div class="close-btn" @click="handleDeleteChat">
      <i class="bx bx-x-circle"></i>
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  value: ChatBox;
  active?: Boolean;
}>();

const emits = defineEmits(["delete"]);
const handleDeleteChat = () => {
  emits("delete", props.value.id);
};
</script>

<style lang="scss" scoped>
.chat-box {
  margin-top: 10px;
  width: 90%;
  height: 70px;
  position: relative;
  border-radius: 20px;
  box-shadow: 0 0 0 0 rgba(0, 0, 0, var(--vs-shadow-opacity));
  border: 2px solid rgba(var(--vs-primary-light-7));

  &:last-child {
    margin-bottom: 10px;
  }

  &:hover {
    border-color: rgba(var(--vs-primary-light-3));
  }

  .title {
    cursor: default;
    height: 40px;
    line-height: 40px;
    text-align: left;
    padding-left: 10px;
    font-size: 20px;
    font-weight: bolder;
    color: black;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }

  .sub-title {
    cursor: default;
    padding-left: 10px;
    padding-right: 10px;
    height: 20px;
    font-size: 12px;
    color: #666;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .count,
    .time {
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }
  }

  .close-btn {
    position: absolute;
    right: 8px;
    top: 5px;
    color: rgba(var(--vs-primary-light-3));
    font-size: 20px;
    opacity: 0;
    transition: all 0.5s ease;
    transition-delay: 0.1s;
    cursor: pointer;

    &:hover {
      transform: translate(1px, -1px);
      color: rgba(var(--vs-primary-dark-2));
    }
  }

  &:hover .close-btn {
    opacity: 1;
  }
}

.active {
  border-color: rgba(var(--vs-primary-light-3));
}
</style>
