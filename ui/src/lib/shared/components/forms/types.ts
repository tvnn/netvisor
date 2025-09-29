import { field, form } from 'svelte-forms';
import { required } from 'svelte-forms/validators';

const dummyField = field('dummy', 'a', [required()]);
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const dummyForm = form();

export type FormType = typeof dummyForm;

export type FieldType = typeof dummyField;

export interface FormApi {
	registerField: (id: string, field: typeof dummyField) => void;
	unregisterField: (id: string) => void;
}
