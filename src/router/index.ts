import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
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
    },
    {
      path: '/quick-add',
      name: 'quick-add',
      component: () => import('../views/QuickAddTask.vue')
    }
  ]
})

export default router