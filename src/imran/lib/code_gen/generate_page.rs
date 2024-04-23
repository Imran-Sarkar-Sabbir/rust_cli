use crate::imran::structs::config::Config;

pub fn generate_page(config: Config) {

}

const TEMPLATE: &str = r#"

import React, {useCallback, useMemo, useState} from 'react';

import {useIntl} from 'react-intl';

import IntlMessages from '@/@softbd/utility-components/IntlMessages';
import PageBlock from '@/@softbd/components/PageBlock';
import AddButton from '@/@softbd/elements/button/AddButton/AddButton';
import DatatableButtonGroup from '@/@softbd/components/DataTable/components/DatatableButtonGroup/DatatableButtonGroup';
import useNotiStack from '@/@softbd/hooks/useNotifyStack';
import IconSkill from '@/@softbd/icons/IconSkill';
import {DatatableFilters} from '@/@softbd/utilities/enums/DataTableFilterEnums';
import {
  isResponseSuccess,
  processCellLimitedString,
} from '@/@softbd/utilities/helpers';
import {permissions} from '@/@softbd/contexts/AllPermissionKeys';
import useCheckPermissions from '@/@softbd/contexts/useCheckPermissions';
import DataTable from '@/@softbd/components/DataTable/Datatable';
import DatatableEditButton from '@/@softbd/components/DataTable/components/buttons/DatatableEditButton';
import DatatableDeleteButton from '@/@softbd/components/DataTable/components/buttons/DatatableDeleteButton';
import useDataTableFetchData from '@/@softbd/hooks/useDataTableFetchData';
import {apiRoutes} from '@/@softbd/common/api-routes';
import DatatableReadButton from '@/@softbd/components/DataTable/components/buttons/DatatableReadButton';
import [[c]]AddEditPopup from './[[c]]AddEditPopup';
import [[c]]DetailsPopup from './[[c]]DetailsPopup';
import {delete[[c]]} from "@/services/clusterManagement/clusterService";

const [[c]]Page = () => {
  const {messages}: any = useIntl();
  const {successStack} = useNotiStack();
  const [selected[[c]]Id, setSelected[[c]]Id] = useState<number | null>(null);
  const [isOpenAddEditModal, setIsOpenAddEditModal] = useState(false);
  const [isOpenDetailsModal, setIsOpenDetailsModal] = useState(false);
  const [isToggleTable, setIsToggleTable] = useState<boolean>(false);
  const {USERS} = permissions;

  const [canRead, canCreate, canUpdate, canDelete] = useCheckPermissions(
    USERS.READ,
    USERS.CREATE,
    USERS.UPDATE,
    USERS.DELETE,
  );

  const {
    onFetchData,
    data: lms,
    loading: isLoading,
    pageCount,
    totalCount,
    mutate: mutate[[c]],
  } = useDataTableFetchData({urlPath: apiRoutes.PRIVATE.CLUSTERS});

  const closeAddEditModal = useCallback(() => {
    setIsOpenAddEditModal(false);
    setSelected[[c]]Id(null);
  }, []);

  const openAddEditModal = useCallback((lmsId: number | null = null) => {
    setIsOpenAddEditModal(true);
    setIsOpenDetailsModal(false);
    setSelected[[c]]Id(lmsId);
  }, []);

  const openDetailsModal = useCallback((itemId: number) => {
    setIsOpenDetailsModal(true);
    setSelected[[c]]Id(itemId);
  }, []);

  const closeDetailsModal = useCallback(() => {
    setIsOpenDetailsModal(false);
  }, []);

  const refreshDataTable = useCallback(() => {
    setIsToggleTable((previousToggle) => !previousToggle);
  }, [isToggleTable]);

  const clusterDelete = async (clusterId: number) => {
    let response = await delete[[c]](clusterId);
    if (isResponseSuccess(response)) {
      successStack(
        <IntlMessages
          id='common.subject_deleted_successfully'
          values={{subject: <IntlMessages id='menu.clusters' />}}
        />,
      );
      refreshDataTable();
    }
  };
  const columns = useMemo(
    () => [
      {
        id: 'title_bn',
        cell: processCellLimitedString('title_bn'),
        enableColumnFilter: true,
        filterFn: DatatableFilters.IncludesString,
        header: messages['common.title_bn'],
      },
      {
        id: 'title_en',
        cell: processCellLimitedString('title_en'),
        enableColumnFilter: true,
        filterFn: DatatableFilters.IncludesString,
        header: messages['common.title_en'],
      },
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
                  deleteAction={() => clusterDelete(data?.id)}
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
          &nbsp;
          <IntlMessages id='menu.clusters' />
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
                  subject: messages['menu.clusters'],
                }}
              />
            }
          />
        ),
      ]}>
      <DataTable
        columns={columns}
        tableData={lms || []}
        fetchData={onFetchData}
        loading={isLoading}
        pageCount={pageCount}
        totalCount={totalCount}
        toggleResetTable={isToggleTable}
        onClickRefresh={mutate[[c]]}
      />

      {isOpenDetailsModal && selected[[c]]Id && (
        <[[c]]DetailsPopup
          key={1}
          itemId={selected[[c]]Id}
          onClose={closeDetailsModal}
          openEditModal={openAddEditModal}
        />
      )}

      {isOpenAddEditModal && (
        <[[c]]AddEditPopup
          key={2}
          clusterId={selected[[c]]Id}
          onClose={closeAddEditModal}
          refreshDataTable={refreshDataTable}
        />
      )}
    </PageBlock>
  );
};

export default [[c]]Page;
"#;