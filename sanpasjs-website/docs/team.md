---
layout: page
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
      { icon: "github", link: "https://github.com/sanjaiyan-dev" },
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
      The development team of <i>SanPasJs</i> which is expected to expand soon 🤩📈🤩
    </template>
  </VPTeamPageTitle>
  <VPTeamMembers
    :members="members"
  />
</VPTeamPage>
