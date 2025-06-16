import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// text after first :
export function displayModelName(name: string) {
  const colonIndex = name.indexOf(':');
  return colonIndex !== -1 ? name.slice(colonIndex + 1).trim() : name;
}
