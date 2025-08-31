import { get, writable } from "svelte/store";

export const loading = writable(false);
export const errorStore = writable<string[]>([]);

export function pushError(message: string, timeout: number = 3000) {
  let errors = get(errorStore);
  errors.push(message);
  console.error(message)
  errorStore.set(errors)

  setInterval(() => {
    errors = get(errorStore)
    errors.filter(e => e !== message);
    errorStore.set(errors)
  }, timeout);

}

export function clearError(message: string) {
  let errors = get(errorStore)
  errors.filter(e => e !== message);
  errorStore.set(errors)
}