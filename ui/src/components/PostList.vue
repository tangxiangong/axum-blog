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
        <div class="post-cover-overlay">
          <span class="read-more">阅读全文</span>
        </div>
      </div>
      <div class="post-content">
        <h2 class="post-title">{{ post.title }}</h2>
        <p class="post-summary">{{ post.summary }}</p>
        <div class="post-meta">
          <span class="post-date">
            <i class="fas fa-calendar-alt"></i>
            {{ post.createdAt }}
          </span>
          <span class="post-author">
            <i class="fas fa-user-circle"></i>
            {{ post.author }}
          </span>
          <span class="post-comments">
            <i class="fas fa-comment-dots"></i>
            {{ post.commentCount }} 评论
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.post-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 1.75rem;
  width: 100%;
  box-sizing: border-box;
}

.post-card {
  width: 100%;
  background: var(--bg-card, white);
  border-radius: 1rem;
  overflow: hidden;
  transition: all 0.3s ease;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05), 0 1px 3px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  position: relative;
  height: 100%;
  cursor: pointer;
}

.post-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.1), 0 5px 15px rgba(0, 0, 0, 0.07);
}

.post-cover {
  width: 100%;
  height: 200px;
  overflow: hidden;
  position: relative;
}

.post-cover img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.5s ease;
}

.post-cover-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.read-more {
  color: white;
  background-color: var(--primary-color, #3b82f6);
  padding: 0.5rem 1rem;
  border-radius: 2rem;
  font-weight: 500;
  font-size: 0.875rem;
  transform: translateY(20px);
  transition: all 0.3s ease;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.25);
}

.post-card:hover .post-cover-overlay {
  opacity: 1;
}

.post-card:hover .read-more {
  transform: translateY(0);
}

.post-card:hover .post-cover img {
  transform: scale(1.1);
}

.post-content {
  padding: 1.5rem;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.post-title {
  margin: 0 0 1rem;
  font-size: 1.25rem;
  color: var(--text-color, #1f2937);
  line-height: 1.4;
  font-weight: 700;
  transition: color 0.2s ease;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-card:hover .post-title {
  color: var(--primary-color, #3b82f6);
}

.post-summary {
  margin: 0 0 1.5rem;
  color: var(--text-secondary, #6b7280);
  line-height: 1.7;
  flex: 1;
  font-size: 0.95rem;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  color: var(--text-secondary, #9ca3af);
  font-size: 0.875rem;
  margin-top: auto;
  border-top: 1px solid var(--border-color, #e5e7eb);
  padding-top: 1rem;
}

.post-meta span {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.post-meta i {
  color: var(--primary-color, #3b82f6);
}

@media (max-width: 768px) {
  .post-list {
    grid-template-columns: 1fr;
  }
  
  .post-cover {
    height: 180px;
  }
}
</style>
