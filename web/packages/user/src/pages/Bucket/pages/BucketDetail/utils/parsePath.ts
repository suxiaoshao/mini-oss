export interface Path {
  folderName: string;
  path: string;
}

export default function parsePath(path: string): Path[] {
  if (path === '/') {
    return [];
  }
  let addPath = '';
  return path
    .split('/')
    .slice(1)
    .map((value) => {
      addPath += `/${value}`;
      return { folderName: value, path: addPath } as Path;
    });
}
