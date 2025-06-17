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

export function mouseEventToRange(e: MouseEvent, el: HTMLElement, min: number, max: number): number {
  const rect = el.getBoundingClientRect();
  const x = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
  return min + (x / rect.width) * (max - min);
}

export function roundToStep(value: number, step: number): number {
  return Math.round(value / step) * step;
}
