type BrandCardProps = {
  name: string;
  statusError?: string;
  os: string;
  product: {
    name: string;
    version: string;
    dataBase: string;
  };
};

export function BrandCard({ name, os, statusError, product }: BrandCardProps) {
  return (
    <div className="content-scene-brandCard">
      <div>
        <p>{`Наименование Виртуальной Машины: ${name}`}</p>
        <p>{`Операционная Cистема: ${os}`}</p>
      </div>
      <div>
        <p></p>
      </div>
    </div>
  );
}
