import { IStandInfo } from "../../models";

type BrandCardProps = {
  standInfo: IStandInfo;
};

/**
"target_server_name": "devalt9.devel.loc",
"target_account": "root",
"instance_dir": "/opt/delo",
"app_name": "delo228",
"db_provider": "PostgreSQL",
"db_server": "localhost",
"db_owner": "X2021",
"db_admin": "postgres",
"web_server_name": "delo228.devel.loc",
"port_a": 10001,
"port_b": 10002,
"smb_server_address": "devalt9.devel.loc"
 */

export function BrandCard({ standInfo }: BrandCardProps) {
  return (
    <div className="content-scene-brandCard">
      <div>
        <p>{`Имя приложения: ${standInfo.web_server_name}`}</p>
      </div>
      <div>
        <p>{`Наименование сервера: ${standInfo.target_server_name}`}</p>
        <p>{`Учетная запись: ${standInfo.target_account}`}</p>
        <p>{`Установочный каталог в системе: ${standInfo.instance_dir}`}</p>
        <p>{`Имя экземпляра приложения стенда: ${standInfo.app_name}`}</p>
        <p>{`Основной порт приложения: ${standInfo.port_a}`}</p>
        <p>{`Вторичный порт приложения (не используется): ${standInfo.port_b}`}</p>
        <p>{`Адрес файл-сервера (SMB): ${standInfo.smb_server_address}`}</p>
      </div>
      <div>
        <ul>
          <li>
            <p>{`Тип используемой СУБД: ${standInfo.db_provider}`}</p>
          </li>
          <li>
            {" "}
            <p>{`Адрес СУБД: ${standInfo.db_server}`}</p>
          </li>
          <li>
            <p>{`Имя пользователя СУБД: ${standInfo.db_owner}`}</p>
          </li>
          <li>
            <p>{`Имя пользователя администратора СУБД: ${standInfo.db_admin}`}</p>
          </li>
        </ul>
      </div>
    </div>
  );
}
