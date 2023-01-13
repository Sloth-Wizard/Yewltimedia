// Import any passed module and maps it by default
export default (...modules: any[]) => Promise.all(
    modules.concat.apply([], modules)
    .map(async (m: any) => (await m).default)
);
