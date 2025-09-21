export interface Group {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  description: string;
  service_bindings: ServiceBinding[];
}

export interface ServiceBinding {
  service_id: string;
  interface_id: string;
}