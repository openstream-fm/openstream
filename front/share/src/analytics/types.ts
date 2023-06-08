export type DataGridField<T> = {
  name: string;
  format: (v: T) => string;
  sort?: ((a: T, b: T) => number) | null;
  numeric?: boolean;
  csv_only?: boolean;
  on_click?: (item: T) => void;
};

export type DataGridData<T, R extends Record<string, any>> = {
  title: string;
  fields: Record<keyof R, DataGridField<T>>;
  items: T[];
  sorted_by?: {
    key: keyof R;
    direction: "asc" | "desc";
  };
};