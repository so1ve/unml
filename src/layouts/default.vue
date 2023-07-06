<template>
  <VApp>
    <VLayout>
      <NavigationDrawer />
      <SystemBar />
      <VMain>
        <div h-full p-4>
          <RouterView />
        </div>
      </VMain>
    </VLayout>
  </VApp>
</template>
