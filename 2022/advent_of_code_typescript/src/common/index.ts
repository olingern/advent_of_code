import { readFileSync } from "fs";

/**
 * Simple sync function to read a file and return the contents as a string
 */
export function readFile(location: string): string {
  return readFileSync(location).toString();
}
