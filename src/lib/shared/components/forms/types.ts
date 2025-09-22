import { field, form } from "svelte-forms";
import { required } from "svelte-forms/validators";

let dummyField = field('dummy', 'a', [required()]);
let dummyForm = form()

export type FormType = typeof dummyForm;

export type FieldType = typeof dummyField;

export interface FormApi {
  registerField: (id: string, field: (typeof dummyField)) => void,
  unregisterField: (id: string) => void
}