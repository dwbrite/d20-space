<template>
  <Login/>
  <MapView/>
</template>

<script>
import MapView from './components/MapView.vue'

export default {
  name: 'App',
  components: {
    MapView
  }
}
</script>

<style>
#app {
  width: 100%;
  height: 100%;
}
</style>
