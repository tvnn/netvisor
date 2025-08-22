<script lang="ts">
	import { diagnosticStatuses, getDiagnosticStatusColor, getDiagnosticStatusIcon } from '$lib/api/registry';
	import { RotateCwSquare } from 'lucide-svelte';
	import { getBgColor, getTextColor } from '../common/colors';
	import { getIcon } from '../common/icons';
	import type { DiagnosticExecution } from './types';
	import Tag from '../common/Tag.svelte';
  
  export let row: DiagnosticExecution;

  $: StatusIcon = getIcon( $getDiagnosticStatusIcon(row.status) );

  console.log(row.status)
  console.log($diagnosticStatuses)
  console.log($getDiagnosticStatusColor(row.status))

</script>

<Tag
  enableIcon={false}
  icon={StatusIcon}
  bgColor={getBgColor($getDiagnosticStatusColor(row.status))}
  textColor={getTextColor($getDiagnosticStatusColor(row.status))}
  label={row.status} />