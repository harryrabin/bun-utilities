/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ProcessResult {
  stdout?: string
  stderr?: string
  exitCode?: number
  isExecuted: boolean
}
export interface Options {
  cwd?: string
  stdin?: 'inherit' | 'piped' | null
  stdout?: 'inherit' | 'piped' | null
  stderr?: 'inherit' | 'piped' | null
}
export function exec(commandWithArgs: Array<string>, options?: Options | undefined | null): { stdout: undefined, stderr: undefined, exitCode?: number, isExecuted: false } | { stdout: string, stderr: string, exitCode?: number, isExecuted: true }
export function spawn(command: string, args: Array<string>, options?: Options | undefined | null): { stdout: undefined, stderr: undefined, exitCode?: number, isExecuted: false } | { stdout: string, stderr: string, exitCode?: number, isExecuted: true }
export interface RecrusiveOptions {
  recursive?: boolean
}
export interface CopyDirOptions {
  recursive?: boolean
  copyFiles?: boolean
}
export function rmdir(path: string, options?: RecrusiveOptions | undefined | null): string
export function copyfile(src: string, dest: string, options?: RecrusiveOptions | undefined | null): string
export function copydir(src: string, dest: string, options?: CopyDirOptions | undefined | null): string
