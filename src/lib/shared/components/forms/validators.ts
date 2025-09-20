import type { Validator } from 'svelte-forms';

// IP Address validator
export const ipAddress = (): Validator => (value: any) => {
  if (!value) return { valid: true, name: 'ipAddress' }; // Allow empty if not required
  
  const ipRegex = /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
  if (!ipRegex.test(value.trim())) {
    return { name: 'invalidIp', message: 'Invalid IP address format', valid: false };
  }

  const octets = value.split('.').map(Number);
  if (octets.some((octet: number) => octet < 0 || octet > 255)) {
      return { name: 'invalidIp', message: 'Invalid IP address range', valid: false };
  }

  return {
    valid: true,
    name: 'validIp',
  };
};

  export const mac = (): Validator => (value: any) => {
    if (!value) return { name: 'validMac', valid: true }; // Optional field
    
    const macRegex = /^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$/;
    if (!macRegex.test(value)) {
        return { name: 'invalidMac', message: 'Invalid MAC address format', valid: false };
    }
    
    return { name: 'validMac', valid: true };
  };

// Hostname validator
export const hostname = (): Validator => (value: string) => {
  if (!value) return { valid: true, name: 'hostname' }; // Allow empty if not required
  
  const hostnameRegex = /^[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?)*$/;
  return {
    valid: hostnameRegex.test(value.trim()),
    name: 'hostname',
    message: 'Please enter a valid hostname or domain'
  };
};

// Max length validator with custom message
export const maxLength = (max: number): Validator => (value: string) => {
  if (!value) return { valid: true, name: 'maxLength' };
  
  return {
    valid: value.length <= max,
    name: 'maxLength',
    message: `Must be less than ${max} characters`
  };
};

// Required field validator for non-editing contexts
export const requiredForNew = (isEditing: boolean): Validator => (value: string) => {
  if (isEditing) return { valid: true, name: 'requiredForNew' };
  
  return {
    valid: Boolean(value && value.trim()),
    name: 'requiredForNew',
    message: 'This field is required for new entries'
  };
};

// Port range validator
export const portRange = (): Validator => (value: number | string) => {
  if (!value && value !== 0) return { valid: true, name: 'portRange' };
  
  const port = typeof value === 'string' ? parseInt(value) : value;
  return {
    valid: Number.isInteger(port) && port >= 1 && port <= 65535,
    name: 'portRange',
    message: 'Port must be between 1 and 65535'
  };
};