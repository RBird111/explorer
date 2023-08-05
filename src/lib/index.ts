export type OsFile = {
	name: string;
	file_type: 'directory' | 'file' | 'symlink';
};

export type Directory = {
	dir: string;
	files: OsFile[];
};
