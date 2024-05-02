use crate::imran::structs::config::Config;
use crate::imran::structs::data_type::Type;

const COLUMN_TEMP: &str = r#"
      {
            id: '[[name]]',
             cell: processCellLimitedString('[[name]]'),
             enableColumnFilter: true,
             filterFn: DatatableFilters.IncludesString,
             header: messages['[[message_key]]'],
      }"#;

fn get_columns(config: &Config) -> Vec<String> {
    let mut columns: Vec<String> = vec![];
    for (key, data_type) in config.properties.iter() {
        match data_type.value.d_type {
            Type::string | Type::number => {
                let column =
                    COLUMN_TEMP.replace("[[name]]", key)
                        .replace("[[message_key]]", &data_type.message_key);
                columns.push(column);
            }
            Type::select => {}
            Type::checkbox => {}
            Type::radio => {}
            Type::none => {
                panic!("type is not allowed.")
            }
        }
    }
    columns
}

pub fn generate_page(config: &Config) -> String {
    let properties = get_columns(config).join(",");
     TEMPLATE.replace("[[capital]]", &config.name.capital)
        .replace("[[upper]]", &config.name.upper)
        .replace("[[camel]]", &config.name.camel)
        .replace("[[snake]]", &config.name.snake)
        .replace("[[columns]]", &properties)
}

const TEMPLATE: &str = r#"
import React, {useCallback, useMemo, useState} from 'react';

import {useIntl} from 'react-intl';

import IntlMessages from '../../../@softbd/utility-components/IntlMessages';
import PageBlock from '../../../@softbd/components/PageBlock';
import AddButton from '../../../@softbd/elements/button/AddButton/AddButton';
import DatatableButtonGroup from '../../../@softbd/components/DataTable/components/DatatableButtonGroup/DatatableButtonGroup';
import useNotiStack from '../../../@softbd/hooks/useNotifyStack';
import IconSkill from '../../../@softbd/icons/IconSkill';
import {DatatableFilters} from '../../../@softbd/utilities/enums/DataTableFilterEnums';
import {
    isResponseSuccess,
    processCellLimitedString,
} from '../../../@softbd/utilities/helpers';
import {permissions} from '../../../@softbd/contexts/AllPermissionKeys';
import useCheckPermissions from '../../../@softbd/contexts/useCheckPermissions';
import DataTable from '../../../@softbd/components/DataTable/Datatable';
import DatatableEditButton from '../../../@softbd/components/DataTable/components/buttons/DatatableEditButton';
import DatatableDeleteButton from '../../../@softbd/components/DataTable/components/buttons/DatatableDeleteButton';
import useDataTableFetchData from '../../../@softbd/hooks/useDataTableFetchData';
import {apiRoutes} from '../../../@softbd/common/api-routes';
import DatatableReadButton from '../../../@softbd/components/DataTable/components/buttons/DatatableReadButton';

import [[capital]]AddEditPopup from './[[capital]]AddEditPopup';
import [[capital]]DetailsPopup from './[[capital]]DetailsPopup';
import {delete[[capital]]} from "@/services/[[camel]]Management/[[camel]]Service";

const [[capital]]Page = () => {
  const {messages}: any = useIntl();
  const {successStack} = useNotiStack();
  const [selected[[capital]]Id, setSelected[[capital]]Id] = useState<number | null>(null);
  const [isOpenAddEditModal, setIsOpenAddEditModal] = useState(false);
  const [isOpenDetailsModal, setIsOpenDetailsModal] = useState(false);
  const [isToggleTable, setIsToggleTable] = useState<boolean>(false);
  const {[[upper]]S} = permissions;

  const [canRead, canCreate, canUpdate, canDelete] = useCheckPermissions(
    [[upper]]S.READ,
    [[upper]]S.CREATE,
    [[upper]]S.UPDATE,
    [[upper]]S.DELETE,
  );

  const {
    onFetchData,
    data: [[camel]]s,
    loading: isLoading,
    pageCount,
    totalCount,
    mutate: mutate[[capital]],
  } = useDataTableFetchData({urlPath: apiRoutes.PRIVATE.[[upper]]S});

  const closeAddEditModal = useCallback(() => {
    setIsOpenAddEditModal(false);
    setSelected[[capital]]Id(null);
  }, []);

  const openAddEditModal = useCallback(([[camel]]Id: number | null = null) => {
    setIsOpenAddEditModal(true);
    setIsOpenDetailsModal(false);
    setSelected[[capital]]Id([[camel]]Id);
  }, []);

  const openDetailsModal = useCallback(([[camel]]Id: number) => {
    setIsOpenDetailsModal(true);
    setSelected[[capital]]Id([[camel]]Id);
  }, []);

  const closeDetailsModal = useCallback(() => {
    setIsOpenDetailsModal(false);
  }, []);

  const refreshDataTable = useCallback(() => {
    setIsToggleTable((previousToggle) => !previousToggle);
  }, [isToggleTable]);

  const [[camel]]Delete = async ([[camel]]Id: number) => {
    let response = await delete[[capital]]([[camel]]Id);
    if (isResponseSuccess(response)) {
      successStack(
        <IntlMessages
          id='common.subject_deleted_successfully'
          values={{subject: <IntlMessages id='menu.[[camel]]s' />}}
        />,
      );
      refreshDataTable();
    }
  };
  const columns = useMemo(
    () => [[[columns]],
      {
        id: 'actions',
        enableColumnFilter: false,
        cell: (props: any) => {
          let data = props.row.original;
          return (
            <DatatableButtonGroup>
              {canRead && (
                <DatatableReadButton
                  onClick={() => openDetailsModal(data?.id)}
                />
              )}
              {canUpdate && (
                <DatatableEditButton
                  onClick={() => openAddEditModal(data?.id)}
                />
              )}
              {canDelete && (
                <DatatableDeleteButton
                  deleteAction={() => [[camel]]Delete(data?.id)}
                />
              )}
            </DatatableButtonGroup>
          );
        },
        header: messages['common.actions'],
        enableHiding: false,
      },
    ],
    [messages, canUpdate, canDelete],
  );
  return (
    <PageBlock
      title={
        <>
          <IconSkill />
          <IntlMessages id='menu.[[snake]]s' />
        </>
      }
      extra={[
        canCreate && (
          <AddButton
            key={1}
            onClick={() => openAddEditModal(null)}
            isLoading={isLoading}
            tooltip={
              <IntlMessages
                id={'common.add_new'}
                values={{
                  subject: messages['menu.[[snake]]s'],
                }}
              />
            }
          />
        ),
      ]}>
      <DataTable
        columns={columns}
        tableData={[[camel]]s || []}
        fetchData={onFetchData}
        loading={isLoading}
        pageCount={pageCount}
        totalCount={totalCount}
        toggleResetTable={isToggleTable}
        onClickRefresh={mutate[[capital]]}
      />

      {isOpenDetailsModal && selected[[capital]]Id && (
        <[[capital]]DetailsPopup
          key={1}
          itemId={selected[[capital]]Id}
          onClose={closeDetailsModal}
          openEditModal={openAddEditModal}
        />
      )}

      {isOpenAddEditModal && (
        <[[capital]]AddEditPopup
          key={2}
          [[camel]]Id={selected[[capital]]Id}
          onClose={closeAddEditModal}
          refreshDataTable={refreshDataTable}
        />
      )}
    </PageBlock>
  );
};

export default [[capital]]Page;
"#;