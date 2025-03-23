import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { adminApi, JWT_TOKEN_KEY } from '@/api'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('@/layouts/BlogLayout.vue'),
    children: [
      {
        path: '',
        name: 'Home',
        component: () => import('@/views/HomeView.vue'),
        meta: {
          title: '博客首页'
        }
      },
      {
        path: 'categories',
        name: 'BlogCategories',
        component: () => import('@/views/HomeView.vue'), // 暂时使用HomeView，后续可以替换为实际的分类页面
        meta: {
          title: '分类'
        }
      },
      {
        path: 'tags',
        name: 'BlogTags',
        component: () => import('@/views/HomeView.vue'), // 暂时使用HomeView，后续可以替换为实际的标签页面
        meta: {
          title: '标签'
        }
      },
      {
        path: 'about',
        name: 'About',
        component: () => import('@/views/AboutView.vue'),
        meta: {
          title: '关于'
        }
      }
    ]
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/Login.vue'),
    meta: {
      title: '登录'
    }
  },
  {
    path: '/admin',
    component: () => import('@/layouts/AdminLayout.vue'),
    meta: {
      requiresAuth: true
    },
    children: [
      {
        path: '',
        redirect: '/admin/dashboard'
      },
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('@/views/admin/Dashboard.vue'),
        meta: {
          title: '仪表盘'
        }
      },
      {
        path: 'website',
        name: 'Website',
        component: () => import('@/views/admin/Website.vue'),
        meta: {
          title: '网站设置'
        }
      },
      {
        path: 'profile',
        name: 'Profile',
        component: () => import('@/views/admin/Profile.vue'),
        meta: {
          title: '个人资料'
        }
      },
      {
        path: 'categories',
        name: 'Categories',
        component: () => import('@/views/admin/CategoryView.vue'),
        meta: {
          title: '分类管理'
        }
      },
      {
        path: 'tags',
        name: 'Tags',
        component: () => import('@/views/admin/TagView.vue'),
        meta: {
          title: '标签管理'
        }
      },
      {
        path: 'articles',
        name: 'Articles',
        component: () => import('@/views/admin/ArticleListView.vue'),
        meta: {
          title: '文章管理'
        }
      },
      {
        path: 'articles/create',
        name: 'CreateArticle',
        component: () => import('@/views/admin/ArticleEditView.vue'),
        meta: {
          title: '写文章'
        }
      },
      {
        path: 'articles/edit/:id',
        name: 'EditArticle',
        component: () => import('@/views/admin/ArticleEditView.vue'),
        meta: {
          title: '编辑文章'
        }
      }
    ]
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/'
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫
router.beforeEach(async (to, from, next) => {
  // 检查用户是否已登录
  const isAuthenticated = await isLoggedIn()

  if (to.matched.some(record => record.meta.requiresAuth)) {
    if (!isAuthenticated) {
      next({
        path: '/login',
        query: { redirect: to.fullPath }
      })
    } else {
      next()
    }
  } else {
    if (isAuthenticated && to.path === '/login') {
      next('/admin')
    } else {
      next()
    }
  }
})

// 检查用户是否已登录
async function isLoggedIn() {
  try {
    // 尝试获取用户信息，如果成功说明用户已登录
    // 确保使用的是正确的 API 调用方法
    await adminApi.getInfo()
    return true
  } catch (error) {
    // 如果请求失败，说明用户未登录或会话已过期
    // 清除可能过期的 token
    localStorage.removeItem(JWT_TOKEN_KEY)
    return false
  }
}

export default router
