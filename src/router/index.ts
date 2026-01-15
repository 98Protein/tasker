import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'task-library',
      component: () => import('../views/TaskLibrary.vue')
    },
    {
      path: '/run-history',
      name: 'run-history',
      component: () => import('../views/RunHistory.vue')
    },
    {
      path: '/system-settings',
      name: 'system-settings',
      component: () => import('../views/SystemSettings.vue')
    }
  ]
})

export default router