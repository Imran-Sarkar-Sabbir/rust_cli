use crate::imran::lib::code_gen::generate_input_fields::generate_input_fields;
use crate::imran::lib::code_gen::generate_input_reset::{get_initial_value, get_input_reset};
use crate::imran::lib::code_gen::generate_yup_validation::get_validation;
use crate::imran::structs::config::Config;

pub fn generate_add_edit(config: &Config) -> String {
    let validation = get_validation(config);
    let reset = get_input_reset(config);
    let initial_values = get_initial_value(config);
    let input_fields = generate_input_fields(config);
    TEMPLATE
        .replace("[[capital]]", &config.name.capital)
        .replace("[[camel]]", &config.name.camel)
        .replace("[[camel]]", &config.name.camel)
        .replace("[[snake]]", &config.name.snake)
        .replace("[[upper]]", &config.name.upper)
        .replace("[[kabeb]]", &config.name.kabeb)
        .replace("[[validation]]", &validation)
        .replace("[[input_fields]]", &input_fields)
        .replace("[[reset]]", &reset)
        .replace("[[initial_values]]", &initial_values)
}


const TEMPLATE: &str = r#"
import React, {FC, useEffect, useMemo} from 'react';
import {yupResolver} from '@hookform/resolvers/yup';
import {Grid} from '@mui/material';
import {SubmitHandler, useForm} from 'react-hook-form';
import {useIntl} from 'react-intl';
import IntlMessages from '../../../@softbd/utility-components/IntlMessages';
import CancelButton from '../../../@softbd/elements/button/CancelButton/CancelButton';
import SubmitButton from '../../../@softbd/elements/button/SubmitButton/SubmitButton';
import useNotiStack from '../../../@softbd/hooks/useNotifyStack';
import useSuccessMessage from '../../../@softbd/hooks/useSuccessMessage';
import IconSkill from '../../../@softbd/icons/IconSkill';
import yup from '../../../@softbd/libs/yup';
import HookFormMuiModal from '../../../@softbd/modals/HookFormMuiModal/HookFormMuiModal';
import {processServerSideErrors} from '../../../@softbd/utilities/validationErrorHandler';
import CustomTextInput from '../../../@softbd/elements/newInput/CustomTextInput';
import FormRowStatus from '../../../@softbd/elements/input/FormRowStatus/FormRowStatus';
import {useFetch[[capital]]ById} from '../../../services/[[camel]]Management/hooks';
import {
  create[[capital]],
  update[[capital]],
} from '../../../services/[[camel]]Management/[[camel]]Service';
import {[[capital]]} from '../../../shared/Interface/common.interface';

interface I[[capital]]AddEditPopup {
  [[camel]]Id: number | null;
  onClose: () => void;
  refreshDataTable: () => void;
}

const initialValues = {
  [[initial_values]]
};
const [[capital]]AddEditPopup: FC<I[[capital]]AddEditPopup> = ({
  [[camel]]Id,
  refreshDataTable,
  ...props
}) => {
  const {messages}: any = useIntl();
  const {errorStack} = useNotiStack();
  const {createSuccessMessage, updateSuccessMessage} = useSuccessMessage();
  const isEdit = [[camel]]Id != null;

  const {data: itemData, isLoading} = useFetch[[capital]]ById([[camel]]Id);

  const validationSchema = useMemo(() => {
    return yup.object().shape({
      [[validation]]
    });
  }, [messages]);

  const {
    register,
    reset,
    handleSubmit,
    setError,
    control,
    formState: {errors, isSubmitting},
  } = useForm<any>({
    resolver: yupResolver(validationSchema),
  });

  useEffect(() => {
    if (itemData) {
      reset({
        [[reset]]
      });
    } else {
      reset(initialValues);
    }
  }, [itemData]);

  const onSubmit: SubmitHandler<any> = async (data) => {
    const formData = {...data};

    try {
      if ([[camel]]Id) {
        await update[[capital]]([[camel]]Id, formData);
        updateSuccessMessage('menu.[[camel]]s');
      } else {
        await create[[capital]](formData);
        createSuccessMessage('menu.[[camel]]s');
      }
      refreshDataTable();
      props.onClose();
    } catch (error: any) {
      processServerSideErrors({error, setError, validationSchema, errorStack});
    }
  };

  return (
    <HookFormMuiModal
      open={true}
      {...props}
      title={
        <>
          <IconSkill />
          {isEdit ? (
            <IntlMessages
              id='common.edit'
              values={{
                subject: <IntlMessages id='menu.[[snake]]s' />,
              }}
            />
          ) : (
            <IntlMessages
              id='common.add_new'
              values={{
                subject: <IntlMessages id='menu.[[snake]]s' />,
              }}
            />
          )}
        </>
      }
      maxWidth={'sm'}
      handleSubmit={handleSubmit(onSubmit)}
      actions={
        <>
          <CancelButton onClick={props.onClose} isLoading={isLoading} />
          <SubmitButton isSubmitting={isSubmitting} isLoading={isLoading} />
        </>
      }>
      <Grid container spacing={3}>
        [[input_fields]]
      </Grid>
    </HookFormMuiModal>
  );
};

export default [[capital]]AddEditPopup;
"#;