import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('_Custom Cases/_Token alma'), [('token') : GlobalVariable.token], FailureHandling.STOP_ON_FAILURE)

// uniqe tel no ve email  oluşturmak için
WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

//println("son 7 rakam : "+ (GlobalVariable.uniqe_id).toString().substring(7))
def RegisterBusinessAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Register_BusinessAccount', 
        [ // zorunlu alanlar
            ('currency_code') : "$currency_code", ('user_email') : ('test' + GlobalVariable.uniqe_id.toString().substring(
                3)) + '@gmail.com', ('user_phone_country_code') : "$user_phone_country_code", ('user_phone_number') : '530' + 
            GlobalVariable.uniqe_id.toString().substring(7), ('tax_office') : "$tax_office", ('tax_number') : GlobalVariable.uniqe_id.toString().substring(
                3 /// 
                ), ('account_number') : "$account_number", ('business_type') : "$business_type", ('name') : "$name", ('alias') : "$alias"
            , ('user_number') : "$user_number", ('user_first_name') : "$user_first_name", ('user_last_name') : "$user_last_name"
            , ('use_iban') : "$use_iban", ('first_name') : "$first_name", ('last_name') : "$last_name", ('email') : "$email"
            , ('address_line1') : "$address_line1", ('address_line2') : "$address_line2", ('zip_postal_code') : "$zip_postal_code"
            , ('phone_number') : "$phone_number", ('state_province_code') : "$state_province_code", ('country_code') : "$country_code"]))

 
// kayıt işleminin başarılı olması beklenir -- olamazsa hata verir
CustomKeywords.'writeFile.WriteFile.writeExel'('Register_BusinessAccount', '', '0', '', '', fileName = 'yeni_tenant_test') 
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : 0]) 
WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'PaymentRegister_BusinessAccount'])

// uniqe tel no oluşturmak için
WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

def Register_PersonalAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Register_PersonalAccount', 
        [ // zorunlu alanlar
            ('currency_code') : "$currency_code", ('user_phone_country_code') : "$user_phone_country_code", ('user_phone_number') : '530' + 
            GlobalVariable.uniqe_id.toString().substring(7 // isteğe bağlı
                ), ('account_number') : "$account_number", ('alias') : "$alias", ('user_number') : "$user_number", ('user_first_name') : "$user_first_name"
            , ('user_last_name') : "$user_last_name", ('user_email') : "$user_email", ('first_name') : "$first_name", ('last_name') : "$last_name"
            , ('email') : "$email", ('address_line1') : "$address_line1", ('address_line2') : "$address_line2", ('zip_postal_code') : "$zip_postal_code"
            , ('state_province_code') : "$state_province_code", ('country_code') : "$country_code"]))

// kayıt işleminin başarılı olması beklenir -- olamazsa hata verir
CustomKeywords.'writeFile.WriteFile.writeExel'('Register_PersonalAccount', '', '0', '', '', fileName = 'yeni_tenant_test')

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : 0])

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'Register_PersonalAccount'])

// SubmitKYCForm (TC No ile) işlemi yapılır -- kullanıcıdan kimlik bilgileri alınır
def kycForm = [('national_id') : '', ('first_name') : '', ('last_name') : '', ('birth_day') : '', ('birth_month') : '', ('birth_year') : '']

BufferedReader br = new BufferedReader(new InputStreamReader(System.in))

print('Personal hesap için tc no \n')

print('Input: national_id?\n')

(kycForm['national_id']) = br.readLine()

print('Input: first_name?\n')

(kycForm['first_name']) = br.readLine()

print('Input: last_name?\n')

(kycForm['last_name']) = br.readLine()

print('Input: birth_day?\n')

(kycForm['birth_day']) = br.readLine()

print('Input: birth_month?\n')

(kycForm['birth_month']) = br.readLine()

print('Input: birth_year?\n')

(kycForm['birth_year']) = br.readLine()

WS.sendRequestAndVerify(findTestObject('Postman/Account/SubmitKYCForm', [('first_name') : kycForm['first_name'], ('last_name') : kycForm[
            'last_name'], ('account_number') : WS.getElementPropertyValue(Register_PersonalAccountResponse, 'payload.account_number')
            , ('national_id') : kycForm['national_id'], ('birth_year') : kycForm['birth_year'], ('birth_day') : kycForm[
            'birth_day'], ('birth_month') : kycForm['birth_month'], ('national_document_type_id') : 1, ('country_code') : "$country_code"]))

CustomKeywords.'writeFile.WriteFile.writeExel'('SubmitKYCForm', '', '0', '', '', fileName = 'yeni_tenant_test')

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : 0])

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'SubmitKYCForm'])

//TopupCash personal hesap için 
def TopupCashResponse =  WS.sendRequestAndVerify(findTestObject('Postman/Transaction/TopupCash', [('tenant_code') : "$tenant_code", ('sender_name') : 'imran'
            , ('sender_iban') : 'TR100006200115900006297322', ('receiver_bank_code') : '10', ('receiver_bank_name') : 'garanti'
            , ('receiver_iban') : 'TR100006200115900006297322', ('amount') : 5, ('currency_code') : "$currency_code", ('description') : WS.getElementPropertyValue(
                Register_PersonalAccountResponse, 'payload.account_number'), ('regex_definition_result_data') : WS.getElementPropertyValue(
                Register_PersonalAccountResponse, 'payload.account_number'), ('bank_transaction_date') : '2022-06-30 17:06'
            , ('bank_receipt_no') : GlobalVariable.uniqe_id, ('national_id_or_tax_no') : kycForm['national_id'], ('wallet_number') : WS.getElementPropertyValue(
                Register_PersonalAccountResponse, 'payload.wallet_number')]))

CustomKeywords.'writeFile.WriteFile.writeExel'('TopupCash', 'personal hesap ile TopupCash', '0', '', '', fileName = 'yeni_tenant_test')

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : 0]) 
WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'TopupCash'])

WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

print('Business hesap için tc no \n')

print('Input: national_id?\n')

(kycForm['national_id']) = br.readLine()

print('Input: first_name?\n')

(kycForm['first_name']) = br.readLine()

print('Input: last_name?\n')

(kycForm['last_name']) = br.readLine()

print('Input: birth_day?\n')

(kycForm['birth_day']) = br.readLine()

print('Input: birth_month?\n')

(kycForm['birth_month']) = br.readLine()

print('Input: birth_year?\n')

(kycForm['birth_year']) = br.readLine()

WS.sendRequestAndVerify(findTestObject('Postman/Account/BusinessAccountSubmitKYCForm', [
		"account_number": WS.getElementPropertyValue( RegisterBusinessAccountResponse, 'payload.account_number') ,
		"first_name": kycForm['first_name'],
		"last_name": kycForm['last_name'],
		"national_id": kycForm['national_id'],
		"birth_year": kycForm['birth_year'],
		"birth_month": kycForm['birth_month'],
		"birth_day": kycForm['birth_day'],
		"national_document_type_id": "1",
		"country_code": "${country_code}" 
	]))

CustomKeywords.'writeFile.WriteFile.writeExel'('BusinessAccountSubmitKYCForm', '', '0', '', '', fileName = 'yeni_tenant_test')

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : 0])

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'BusinessAccountSubmitKYCForm'])


/*
// Business hesap için TopupCash işlemi tanımlı olmadığında hata alınıyor sonraki işlem yapılmıyor
//TopupCash Business hesap için
TopupCashResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/TopupCash', [('tenant_code') : "$tenant_code", ('sender_name') : 'imran'
            , ('sender_iban') : 'TR100006200115900006297322', ('receiver_bank_code') : '10', ('receiver_bank_name') : 'garanti'
            , ('receiver_iban') : 'TR100006200115900006297322', ('amount') : 5, ('currency_code') : "$currency_code", ('description') : WS.getElementPropertyValue(
                RegisterBusinessAccountResponse, 'payload.account_number'), ('regex_definition_result_data') : WS.getElementPropertyValue(
                RegisterBusinessAccountResponse, 'payload.account_number'), ('bank_transaction_date') : '2022-06-30 17:06'
            , ('bank_receipt_no') : GlobalVariable.uniqe_id, ('national_id_or_tax_no') : kycForm['national_id'], ('wallet_number') : WS.getElementPropertyValue(
                RegisterBusinessAccountResponse, 'payload.wallet_number')]))


CustomKeywords.'writeFile.WriteFile.writeExel'('TopupCash', 'Business hesap ile TopupCash', '0', '', '', fileName = 'yeni_tenant_test') 
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : 0])
*/


// payment işlemi 

WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
 
WS.sendRequestAndVerify(findTestObject('Postman/Transaction/Payment',[
		"ext_transaction_id": GlobalVariable.uniqe_id,
		"sender_account_number": WS.getElementPropertyValue( Register_PersonalAccountResponse, 'payload.account_number') ,
		 "sender_wallet_number": WS.getElementPropertyValue(  Register_PersonalAccountResponse, 'payload.wallet_number'),
		 "currency_code": "${currency_code}",
		 "amount": 1,
		 "description": "",
		 "receiver_wallet_number": WS.getElementPropertyValue( RegisterBusinessAccountResponse, 'payload.wallet_number') 
	]))

CustomKeywords.'writeFile.WriteFile.writeExel'('Payment', '', '0', '', '', fileName = 'yeni_tenant_test') 
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : 0])

