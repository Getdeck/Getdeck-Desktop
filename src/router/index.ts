// Composables
import { createRouter, createWebHistory } from 'vue-router';
const routes = [
    {
        path: '/',
        component: () => import('@/layouts/default/Default.vue'),
        children: [
            {
                path: '/home',
                name: 'Home',
                // route level code-splitting
                // this generates a separate chunk (about.[hash].js) for this route
                // which is lazy-loaded when the route is visited.
                component: () => import(/* webpackChunkName: "home" */ '@/views/Home.vue'),
            },
            {
                path: '/clusters',
                name: 'Clusters',
                // route level code-splitting
                // this generates a separate chunk (about.[hash].js) for this route
                // which is lazy-loaded when the route is visited.
                component: () => import(/* webpackChunkName: "home" */ '@/views/Clusters.vue'),
            },
            {
                path: '/clusters/create',
                name: 'Cluster Create',
                // route level code-splitting
                // this generates a separate chunk (about.[hash].js) for this route
                // which is lazy-loaded when the route is visited.
                component: () => import(/* webpackChunkName: "home" */ '@/views/ClusterCreate.vue'),
            },
            {
                path: '/login',
                name: 'Login',
                component: () => import('@/views/Login.vue'),
            },
        ],
    },
];
const router = createRouter({
    history: createWebHistory(),
    routes,
});
export default router;
