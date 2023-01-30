<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TopupCash</name>
   <tag></tag>
   <elementGuidId>db66ff47-d3af-4d56-bab2-c3de7cd17cfe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;tenant_code\&quot;: \&quot;${tenant_code}\&quot;,\n    \&quot;sender_name\&quot;: \&quot;${sender_name}\&quot;,\n    \&quot;sender_iban\&quot;: \&quot;${sender_iban}\&quot;, \n    \&quot;receiver_bank_code\&quot;: \&quot;${receiver_bank_code}\&quot;, \n    \&quot;receiver_bank_name\&quot;: \&quot;${receiver_bank_name}\&quot;,\n    \&quot;receiver_iban\&quot;: \&quot;${receiver_iban}\&quot;, \n    \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n    \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n    \&quot;description\&quot;: \&quot;${description}\&quot;,  \n    \&quot;regex_definition_result_data\&quot;: \&quot;${regex_definition_result_data}\&quot;,\n    \&quot;bank_transaction_date\&quot;: \&quot;${bank_transaction_date}\&quot;,\n    \&quot;bank_receipt_no\&quot;: \&quot;${bank_receipt_no}\&quot;,\n    \&quot;national_id_or_tax_no\&quot;: \&quot;${national_id_or_tax_no}\&quot;,\n  \&quot;wallet_number\&quot;:\&quot;${wallet_number}\&quot;,\n    \&quot;national_id_hashed\&quot;: \&quot;${national_id_hashed}\&quot;,\n    \&quot;iban_hashed\&quot;: \&quot;${iban_hashed}\&quot;,\n    \&quot;source_type\&quot;: \&quot;${source_type}\&quot;,\n    \&quot;channel_type\&quot;: \&quot;${channel_type}\&quot;,\n    \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;\n}&quot;,
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
      <webElementGuid>09c5981b-1539-4c80-94a2-8085d994db87</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>55390e09-1897-4579-9fd5-079b53d7746d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/TopupCash</restUrl>
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
      <id>6261471a-fe0b-4bca-9050-22cbdc7a86e0</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>13e52727-4fdc-401d-83f0-202258fbae8e</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4d854be5-cc0f-42ca-98d1-1b92f373dccd</id>
      <masked>false</masked>
      <name>tenant_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ab7dd751-5805-4cad-99ce-5c13734c0651</id>
      <masked>false</masked>
      <name>sender_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>04d2d700-7130-45f3-b82b-64a74f736ef9</id>
      <masked>false</masked>
      <name>sender_iban</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>32336720-a0af-4207-9593-2e6a242aa473</id>
      <masked>false</masked>
      <name>receiver_bank_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2bb2ee93-af2a-40f6-86dc-dd789d28174e</id>
      <masked>false</masked>
      <name>receiver_bank_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>94dfbd50-3b39-434b-bba3-f6f5f7cb7851</id>
      <masked>false</masked>
      <name>receiver_iban</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>b1cf3308-ce83-46a4-a363-bb7de9c00318</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e9dfc019-4308-49ea-8ab6-be473bf5853a</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>hesap no girilmeli</description>
      <id>a5b155cf-94c1-4771-b4c2-b35c38c12810</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>19a7baab-40b5-4d68-bb78-1af3984d26ce</id>
      <masked>false</masked>
      <name>regex_definition_result_data</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>28ed748e-2bda-40a3-ac14-cdab7f3ed231</id>
      <masked>false</masked>
      <name>bank_transaction_date</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>f7d2a310-0a6e-4ac6-8814-367e37ca146c</id>
      <masked>false</masked>
      <name>bank_receipt_no</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2db8d0df-4677-4bce-9bd9-6d0a2b907421</id>
      <masked>false</masked>
      <name>national_id_or_tax_no</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>45d26b0c-e89a-4903-a986-ec160111c805</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>404f8b50-ad67-4e2e-8b03-5d153423f6d2</id>
      <masked>false</masked>
      <name>national_id_hashed</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>76f84421-f6cf-417c-b5c3-9f452dcf4c07</id>
      <masked>false</masked>
      <name>iban_hashed</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>382f2e18-38d7-47ed-b0dc-30e84ff89fbe</id>
      <masked>false</masked>
      <name>source_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a564493f-f8bc-4891-b122-65f34ce2bf20</id>
      <masked>false</masked>
      <name>channel_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ddfce75c-fa7d-4fab-985e-e78ef05a868a</id>
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

println(&quot;TopupCash request body: \n&quot;+ request.getBodyContent().getText())


WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)



import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText());
println(&quot;TopupCash response body\n&quot;+json.toString(4)); 

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
