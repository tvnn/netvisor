<script lang="ts">
  import type { Node, AssignedTest } from "$lib/types/nodes";
  import { createEmptyNodeFormData, nodeToFormData, formDataToNodeApi } from "$lib/types/nodes";
  import { nodeActions } from '$lib/stores/nodes';

  export let formData;

</script>

<div>
    <label for="monitoring_interval" class="block text-sm font-medium text-gray-300 mb-1">
        Monitoring Interval (minutes)
    </label>
    <input
        id="monitoring_interval"
        name="monitoring_interval"
        bind:value={formData.monitoring_interval}
        type="number"
        min="0"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        placeholder="10"
    />
    <p class="text-xs text-gray-400 mt-1">
        Set to 0 to disable monitoring, or specify interval in minutes.
    </p>
</div>

