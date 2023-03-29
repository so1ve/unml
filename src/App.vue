<template>
  <VApp>
    <VLayout>
      <VNavigationDrawer
        class="bg-u-black!"
        :elevation="0"
        model-value
        permanent
        rail
      >
        <VListItem
          nav
          prepend-avatar="https://randomuser.me/api/portraits/men/85.jpg"
        >
          <template #append>
            <VBtn
              icon="mdi-chevron-left"
              variant="text"
            />
          </template>
        </VListItem>

        <VDivider />

        <VList density="compact" nav>
          <VListItem prepend-icon="mdi-home-city" value="home" />
          <VListItem prepend-icon="mdi-account" value="account" />
          <VListItem prepend-icon="mdi-account-group-outline" value="users" />
        </VList>
      </VNavigationDrawer>
      <SystemBar />
      <VMain class="main">
        <div class="p-4">
          <RouterView />
        </div>
      </VMain>
    </VLayout>
  </VApp>
</template>
