---
layout: page
title: Team
---

<script setup>
import {
  VPTeamPage,
  VPTeamPageTitle,
  VPTeamMembers
} from 'vitepress/theme'

const members = [
  {
    avatar: '/TeamPics/SanjaiyanParthipan.jpeg',
    name: 'Sanjaiyan Parthipan',
    title: 'Creator',
    links: [
      { icon: "instagram", link: "https://instagram.com/sanjaiyan_dev" },
    ]
  },
]
</script>

<VPTeamPage>
  <VPTeamPageTitle>
    <template #title>
      Our Team
    </template>
    <template #lead>
      The development team of <i>SanPasJs</i> which is expected to grow soon 🤩📈🤩
    </template>
  </VPTeamPageTitle>
  <VPTeamMembers
    :members="members"
  />
</VPTeamPage>

<style>
    * {
        scroll-behavior: smooth;
    }
</style>
