<script setup lang="ts">
interface Post {
  id: number;
  title: string;
  summary: string;
  cover: string;
  createdAt: string;
  author: string;
  commentCount: number;
}

defineProps<{
  posts: Post[]
}>()
</script>

<template>
  <div class="post-list">
    <div class="post-card" v-for="post in posts" :key="post.id">
      <div class="post-cover">
        <img :src="post.cover" :alt="post.title">
      </div>
      <div class="post-content">
        <h2 class="post-title">{{ post.title }}</h2>
        <p class="post-summary">{{ post.summary }}</p>
        <div class="post-meta">
          <span class="post-date">
            <i class="fas fa-calendar"></i>
            {{ post.createdAt }}
          </span>
          <span class="post-author">
            <i class="fas fa-user"></i>
            {{ post.author }}
          </span>
          <span class="post-comments">
            <i class="fas fa-comments"></i>
            {{ post.commentCount }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.post-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  width: 100%;
  box-sizing: border-box;
}

.post-card {
  width: 100%;
  background: white;
  border-radius: 0.75rem;
  overflow: hidden;
  transition: transform 0.2s, box-shadow 0.2s;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

.post-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}

.post-cover {
  width: 100%;
  height: 240px;
  overflow: hidden;
}

.post-cover img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s;
}

.post-card:hover .post-cover img {
  transform: scale(1.05);
}

.post-content {
  padding: 1.25rem;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.post-title {
  margin: 0 0 0.75rem;
  font-size: 1.25rem;
  color: var(--text-color);
  line-height: 1.4;
}

.post-summary {
  margin: 0 0 1rem;
  color: #718096;
  line-height: 1.6;
  flex: 1;
}

.post-meta {
  display: flex;
  gap: 1rem;
  color: #a0aec0;
  font-size: 0.875rem;
  margin-top: auto;
}

.post-meta span {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.post-meta i {
  font-size: 0.875rem;
}
</style>
