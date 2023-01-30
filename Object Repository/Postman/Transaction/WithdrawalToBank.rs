<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>WithdrawalToBank</name>
   <tag></tag>
   <elementGuidId>0306cbaf-b61c-4cce-8ee9-e3da19bc8533</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ext_transaction_id\&quot;:\&quot;${ext_transaction_id}\&quot;,\n  \&quot;receiver_account_number\&quot;:\&quot;${receiver_account_number}\&quot;,\n  \&quot;receiver_wallet_number\&quot;:\&quot;${receiver_wallet_number}\&quot;,\n  \&quot;receiver_national_id\&quot;:\&quot;${receiver_national_id}\&quot;,\n  \&quot;receiver_account_holder_name\&quot;:\&quot;${receiver_account_holder_name}\&quot;,\n  \&quot;receiver_iban\&quot;:\&quot;${receiver_iban}\&quot;,\n  \&quot;description\&quot;:\&quot;${description}\&quot;,\n  \&quot;amount\&quot;:\&quot;${amount}\&quot;,\n  \&quot;currency_code\&quot;:\&quot;${currency_code}\&quot;,\n  \&quot;is_save_bank_account\&quot;:\&quot;${is_save_bank_account}\&quot;,\n  \&quot;bank_account_name\&quot; : \&quot;${bank_account_name}\&quot;,\n  \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;\n}\n\n &quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>74a98e42-a24e-4235-ba2f-88693b19f81a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>424c0720-66a6-4b88-be29-c8bc15c9c9e0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/WithdrawalToBank</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>06d041e3-a294-4536-b659-802fb62ac280</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>63cf45f0-bb21-49cc-96ab-dde18558c536</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>20692e4b-3a17-4a06-8d26-30f538355e36</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b75997e4-c03d-491f-b1d0-59d955519a58</id>
      <masked>false</masked>
      <name>receiver_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c2b8a7e6-bd78-435a-a6c2-b3716a8146b0</id>
      <masked>false</masked>
      <name>receiver_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5f710a4e-0ddc-4514-b9cd-5f1c0df63d1a</id>
      <masked>false</masked>
      <name>receiver_account_holder_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bac43cda-a2d2-4565-94ec-bbfdd0803b68</id>
      <masked>false</masked>
      <name>receiver_national_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>265dad79-5301-4c76-b1af-fb4a8fc9a8c9</id>
      <masked>false</masked>
      <name>receiver_iban</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c8f0e9f6-714d-4286-b689-4aafb56c0f78</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ae92e0cb-058f-47da-bdf4-b085368ed5a0</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ee036ad4-1f55-44e2-aaa5-a253c6459e58</id>
      <masked>false</masked>
      <name>is_save_bank_account</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1b78be18-1bdc-458a-a67f-d432f9042432</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cb1eb793-c6fe-438b-aea6-7b6993ee5763</id>
      <masked>false</masked>
      <name>bank_account_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ada6ff6d-98b6-4ce2-9df0-fe60a9c306a1</id>
      <masked>false</masked>
      <name>hash_key</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


GlobalVariable.responseObject = response
GlobalVariable.requestObject = request

println(&quot;WithdrawalToBank request body: \n&quot;+ request.getBodyContent().getText())


WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;WithdrawalToBank response body\n&quot;+json.toString(4));


 
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
