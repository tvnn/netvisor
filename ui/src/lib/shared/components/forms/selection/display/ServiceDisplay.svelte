<script lang="ts" context="module">
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';

	export const ServiceDisplay: EntityDisplayComponent<Service> = {
		getId: (service: Service) => service.id,
		getLabel: (service: Service) => service.name,
		getDescription: (service: Service) => {
			let binding_count = service.bindings.length;
			return binding_count + ' binding' + (binding_count == 1 ? '' : 's');
		},
		getIcon: (service: Service) => serviceDefinitions.getIconComponent(service.service_definition),
		getIconColor: (service: Service) =>
			serviceDefinitions.getColorHelper(service.service_definition).icon,
		getTags: (service: Service) => {
			let tags = [];

			if (service.virtualization) {
				const tag: TagProps = {
					label: service.virtualization.type,
					color: entities.getColorHelper('Virtualization').string
				};

				tags.push(tag);
			}

			if (service.source.type == 'Discovery' && service.source.metadata.result_details) {
				let confidence = service.source.metadata.result_details.confidence;

				const tag: TagProps = {
					label: confidence + ' Confidence',
					color: confidence === 'High' ? 'green' : confidence === 'Medium' ? 'blue' : 'yellow'
				};

				tags.push(tag);
			}

			return tags;
		},
		getIsDisabled: () => false,
		getCategory: () => null
	};
</script>

<script lang="ts">
	import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import type { EntityDisplayComponent } from '../types';
	import type { Service } from '$lib/features/services/types/base';
	import type { TagProps } from '$lib/shared/components/data/types';

	export let item: Service;
</script>

<ListSelectItem {item} displayComponent={ServiceDisplay} />
