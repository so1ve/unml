<template>
  <VApp>
    <VLayout>
      <NavigationDrawer />
      <SystemBar />
      <VMain>
        <div class="p-4">
          <RouterView />
        </div>
      </VMain>
    </VLayout>
  </VApp>
</template>
