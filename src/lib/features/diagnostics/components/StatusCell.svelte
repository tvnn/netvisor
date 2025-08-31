<script lang="ts">
	import type { DiagnosticExecution } from '../types/base';
	import { diagnosticStatuses } from '$lib/shared/stores/registry';
	import Tag from '$lib/shared/components/data/Tag.svelte';
  
  export let row: DiagnosticExecution;

  $: styles = diagnosticStatuses.getStyle(row.status);

</script>

<Tag
  enableIcon={false}
  icon={styles.IconComponent}
  bgColor={styles.colors.bg}
  textColor={styles.colors.text}
  label={row.status} />