/* tslint:disable */
/* eslint-disable */

/**
 * Apply the default embedded ICC profile to a PNG image
 *
 * The library includes a pre-defined ICC profile that can be applied
 * to any PNG image without needing to provide an external profile.
 *
 * # Arguments
 * * `image_data` - The PNG image data as bytes
 *
 * # Returns
 * The PNG image with the default ICC profile embedded
 */
export function apply_default_icc_profile(image_data: Uint8Array): Uint8Array;

/**
 * Apply an ICC profile to a PNG image
 *
 * # Arguments
 * * `image_data` - The PNG image data as bytes
 * * `icc_profile` - The ICC profile data as bytes
 *
 * # Returns
 * A new PNG image with the ICC profile embedded
 */
export function apply_icc_profile(image_data: Uint8Array, icc_profile: Uint8Array): Uint8Array;

/**
 * Extract ICC profile from a PNG image
 *
 * # Arguments
 * * `image_data` - The PNG image data as bytes
 *
 * # Returns
 * The ICC profile data if present, or an empty vector if not
 */
export function extract_icc_profile(image_data: Uint8Array): Uint8Array;

/**
 * Get the embedded default ICC profile
 *
 * Returns a copy of the ICC profile that is statically included in the library.
 *
 * # Returns
 * The default ICC profile data as bytes
 */
export function get_default_icc_profile(): Uint8Array;

/**
 * Apply ICC profile from one image to another
 *
 * # Arguments
 * * `source_image` - The PNG image to extract the ICC profile from
 * * `target_image` - The PNG image to apply the ICC profile to
 *
 * # Returns
 * The target image with the ICC profile from the source image
 */
export function transfer_icc_profile(source_image: Uint8Array, target_image: Uint8Array): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly apply_icc_profile: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly extract_icc_profile: (a: number, b: number) => [number, number, number, number];
  readonly transfer_icc_profile: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly apply_default_icc_profile: (a: number, b: number) => [number, number, number, number];
  readonly get_default_icc_profile: () => [number, number];
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
