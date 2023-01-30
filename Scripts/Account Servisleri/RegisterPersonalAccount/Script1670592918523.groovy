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

import customKeyword.CustomMethods as Custom
import com.kms.katalon.core.testobject.ResponseObject
import customKeyword.Verify as Verify

String body = """{
    'account_number': $account_number,
    'currency_code': $currency_code,
    'alias': $alias,
    'use_iban': $use_iban,
    'user_number':$user_number,

    'group_code': $group_code,

    'user_first_name': $user_first_name,
    'user_last_name': $user_last_name,
    'user_phone_country_code': $user_phone_country_code,
    'user_phone_number': $user_phone_number,
    'user_email': $user_email,
    'contact_address':{
        'first_name': $first_name,
        'last_name': $last_name,
        'email':$email,
        'address_line1': $address_line1,
        'address_line2': $address_line2,
        'zip_postal_code': $zip_postal_code,
        'phone_number': $phone_number,
        'state_province_code': $state_province_code,
        'country_code': $country_code
        }
}"""




ResponseObject response = Custom.sendRequest('/v1/Account/RegisterPersonalAccount', body)

 
Verify.VerifyResponseStatusCode("$expected_ResponseStatusCode",response)

Verify.VerifyElementPropetyValue("$expected_message", "message", response)
Verify.VerifyElementPropetyValue("$expected_status", "status", response)
Verify.VerifyElementPropetyValue("$expected_code", "code", response)



//println(body)

