import { api } from "$lib/shared/utils/api";
import { writable } from "svelte/store";
import { type CapabilityFormRequest } from "./types/api";
import type { CapabilityConfigForm } from "./types/forms";

export const forms = writable<Record<string, CapabilityConfigForm>>({});

export async function getCapabilityForms(data: CapabilityFormRequest) {
  return await api.request<Record<string, CapabilityConfigForm>>(
    '/capabilities/forms',
    forms,
    (forms) => forms,
    {
      method: 'POST',
      body: JSON.stringify(data)
    },
    true
  )
}