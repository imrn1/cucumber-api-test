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

def results

def accountType = 0

def name = "$name".trim()

// mail adresiden kyc10_b_im_1@mail.com kalıbın alınması gerekli ***********


accountType = (name.split('_')[1]).toLowerCase() == 'p' ? '1' : '2' // kyc10_b_im_1  

//println("name :"+name)
//println("accountType : "+ accountType)

name = name + "@mail.com"
//println("name :"+name)


if (accountType == '1') {
    // personal hesap 
    // personal list by filter çağırılıp account number alınmalı -- ilk elemandan  -- default_account_number >>  global değişkene atanmalı
    def pListResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/PersonalListByFilter', [
			('id') : '0', 
			('tenant_id') : '0',
			('account_id') : '0', 
			('account_number') : '', 
			//('first_name') : "$name",   // first_name alanı kyc yapıldığında değişmekte 
			('first_name') : '',
			('last_name') : '', ('end_date') : '',
	        ('start_date') : '', 
			('access_level_status_id') : '', 
			('kyc_level') : '', 
			('phone_number') : '', 
			('email') : "$name",
			('phone_country_code') : '', 
			('national_id') : '', 
			('page_size') : '50', 
			('page_index') : '1', 
			('order_column') : 'Id',
			('order_by') : 'asc'
		]))

    results = WS.getElementPropertyValue(pListResponse, 'payload.results')
	
	//println("results size = "+results.size)
	WS.verifyGreaterThan(results.size, 0)

    GlobalVariable.account_number = ((results[0])['account_number'])
	
	
	// payload.results[0].user_kyc_info.national_id
	GlobalVariable.national_id = results[0]['user_kyc_info']['national_id'] 
	println("getaccount servisi --> national_id : "+ GlobalVariable.national_id)
	

    /*for(def item : results ) {
		println("account_number : "+ item["account_number"])
		println("f name : "+ item["user_kyc_info"]["first_name"])
	}*/
    // check personal wallet ile cüzdan no alınıp global değişkene atanmalı
    def CheckPersonalWalletResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/CheckPersonalWallet',
		 [('account_number') : GlobalVariable.account_number
                , ('currency_code') : "$currency_code"]))

    GlobalVariable.wallet_number = WS.getElementPropertyValue(CheckPersonalWalletResponse, 'payload.wallet_number') //println("wallet number : "+ GlobalVariable.default_sender_wallet_number  )
    // business hesap
} else if (accountType == '2') {
    def bListResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/BusinessListByFilter', [
			('tanant_id') : 0
	        , ('page_size') : 5, ('page_index') : 1, 
			'order_column' : '"' + 'Id' + '"', ('order_by') : ('"' + 'asc') + '"',
			"email": '"' + "$name" + '"'
		]))

    results = WS.getElementPropertyValue(bListResponse, 'payload.results')
	//println("results size = "+results.size)
	WS.verifyGreaterThan(results.size, 0)
	
    GlobalVariable.account_number = ((results[0])['account_number'])

    //println('b account number : ' + GlobalVariable.account_number)

    def CheckBusinessWalletResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/CheckBusinessWallet', 
		[('account_number') : GlobalVariable.account_number
                , ('currency_code') : "$currency_code"]))

    GlobalVariable.wallet_number = WS.getElementPropertyValue(CheckBusinessWalletResponse, 'payload.wallet_number')

	GlobalVariable.national_id = ""   // işletme hesaplarrına ait nationl_id dönmemekte  !!!
	
  //  println('b wallet number : ' + GlobalVariable.wallet_number)
}

